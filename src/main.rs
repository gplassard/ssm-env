use std::{collections::HashMap, process::ExitStatus};
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;
use aws_sdk_ssm::{Client, Error as SsmError};

use crate::cli::SubCommand;
use clap::Parser;
use cli::Cli;
use env_logger::Builder;
use log::{debug, info};

mod cli;

#[::tokio::main]
async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    Builder::new().filter_level(cli.log_level).init();

    debug!("retrieving AWS credentials");
    let aws_config = aws_config::load_from_env().await;
    let ssm_client = Client::new(&aws_config);


    match cli.command {
        SubCommand::Exec { ssm_path_prefix, command, args } => {
            let env_variables = fetch_ssm_parameters(ssm_client, ssm_path_prefix).await?;
            exec(command, args, env_variables)?
        },
        SubCommand::ExecAnsibleVaultMode { ssm_path, command, args } => {
            let secret = fetch_ssm_parameter(ssm_client, ssm_path).await?;
            let mut vault_file = NamedTempFile::new()?;
            let vault_file_name = &vault_file.path().to_str().unwrap_or("ssm-env-default").to_string();
            info!("Temporary file {:?} created", &vault_file_name);
            vault_file.write_all(secret.unwrap().as_bytes())?;
            let env_variables: HashMap<String, String> = [("DEFAULT_VAULT_PASSWORD_FILE".to_string(), "".to_string())].iter().cloned().collect();
            let res = exec(command, args, env_variables)?;
            vault_file.close()?;
            info!("Temporary file {:?} cleaned", &vault_file_name);
            res
        },
    };
    Ok(())
}

async fn fetch_ssm_parameters(ssm_client: Client, path_prefix: String) -> Result<HashMap<String, String>, SsmError> {
    debug!("retrieving SSM parameters from path {}", &path_prefix);
    let result = ssm_client
        .get_parameters_by_path()
        .path(&path_prefix)
        .send()
        .await?;
    debug!("SSM parameters retrieved");

    let env_variables: HashMap<String, String> = result
        .parameters()
        .iter()
        .filter_map(|parameter| match (parameter.name(), parameter.value()) {
            (Some(name), Some(value)) => {
                Some((name.to_string().replacen(&path_prefix, "", 1), value.to_string()))
            }
            _ => None,
        })
        .collect();
    Ok(env_variables)
}
async fn fetch_ssm_parameter(ssm_client: Client, path: String) -> Result<Option<String>, SsmError> {
    debug!("retrieving SSM parameter {}", &path);
    let result = ssm_client
        .get_parameter()
        .name(&path)
        .send()
        .await?;
    debug!("SSM parameter retrieved");

    Ok(result.parameter.map(|p| p.value).flatten())
}

fn exec(command: String, args: Vec<String>, env_variables: HashMap<String, String>) -> Result<ExitStatus, CliError>{
    info!(
        "The following environment variables will be exposed {:?}",
        env_variables.keys()
    );
    info!("Executing {} with args {:?}", command, args);
    let mut sub = Command::new(command)
        .args(args)
        .envs(env_variables)
        .spawn()?;

    sub.wait()
        .map_err(|e| e.into())
}

#[derive(Debug)]

struct CliError {
    message: String,
    error: Box<dyn std::error::Error>,
}


impl From<SsmError> for CliError {
    fn from(error: SsmError) -> Self {
        CliError {
            message: "AWS SSM error".to_string(),
            error: Box::new(error),
        }
    }
}
impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError {
            message: "IO error".to_string(),
            error: Box::new(error),
        }
    }
}