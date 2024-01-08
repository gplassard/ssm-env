use std::collections::HashMap;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;
use aws_sdk_ssm::{Client, Error};

use crate::cli::SubCommand;
use clap::Parser;
use cli::Cli;
use env_logger::Builder;
use log::{debug, info};

mod cli;

#[::tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    Builder::new().filter_level(cli.log_level).init();

    debug!("retrieving AWS credentials");
    let aws_config = aws_config::load_from_env().await;
    let ssm_client = Client::new(&aws_config);


    match cli.command {
        SubCommand::Exec { ssm_path_prefix, command, args } => {
            let env_variables = fetch_ssm_parameters(ssm_client, ssm_path_prefix).await?;
            exec(command, args, env_variables)
        },
        SubCommand::ExecAnsibleVaultMode { ssm_path, command, args } => {
            let secret = fetch_ssm_parameter(ssm_client, ssm_path).await?;
            let mut vault_file = NamedTempFile::new().unwrap();
            let vault_file_name = &vault_file.path().to_str().unwrap().to_string();
            info!("Temporary file {:?} created", &vault_file_name);
            vault_file.write_all(secret.unwrap().as_bytes()).unwrap();
            let env_variables: HashMap<String, String> = [("DEFAULT_VAULT_PASSWORD_FILE".to_string(), "".to_string())].iter().cloned().collect();
            let res = exec(command, args, env_variables);
            vault_file.close().unwrap();
            info!("Temporary file {:?} cleaned", &vault_file_name);
            res
        },
    };
    Ok(())
}

async fn fetch_ssm_parameters(ssm_client: Client, path_prefix: String) -> Result<HashMap<String, String>, Error> {
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
async fn fetch_ssm_parameter(ssm_client: Client, path: String) -> Result<Option<String>, Error> {
    debug!("retrieving SSM parameter {}", &path);
    let result = ssm_client
        .get_parameter()
        .name(&path)
        .send()
        .await?;
    debug!("SSM parameter retrieved");

    Ok(result.parameter.map(|p| p.value).flatten())
}

fn exec(command: String, args: Vec<String>, env_variables: HashMap<String, String>) {
    info!(
        "The following environment variables will be exposed {:?}",
        env_variables.keys()
    );
    info!("Executing {} with args {:?}", command, args);
    Command::new(command)
        .args(args)
        .envs(env_variables)
        .spawn()
        .expect("Error spawning command")
        .wait()
        .expect("Command failed");
}
