use std::collections::HashMap;
use std::process::Command;

use aws_sdk_ssm as ssm;

use crate::cli::SubCommand;
use clap::Parser;
use env_logger::Builder;
use log::{debug, info};
use cli::Cli;

mod cli;

#[::tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<(), ssm::Error> {
    let cli = Cli::parse();
    Builder::new().filter_level(cli.log_level).init();

    debug!("retrieving AWS credentials");
    let aws_config = aws_config::load_from_env().await;
    let ssm_client = aws_sdk_ssm::Client::new(&aws_config);

    let path = cli.path;
    debug!("retrieving SSM parameters from path {}", &path);
    let result = ssm_client
        .get_parameters_by_path()
        .path(&path)
        .send()
        .await?;
    debug!("SSM parameters retrieved");

    let env_variables: HashMap<String, String> = result
        .parameters()
        .iter()
        .filter_map(|parameter| match (parameter.name(), parameter.value()) {
            (Some(name), Some(value)) => Some((name.to_string().replacen(&path, "", 1), value.to_string())),
            _ => None,
        })
        .collect();

    match cli.command {
        SubCommand::Exec { command, args } => exec(command, args, env_variables),
    };
    Ok(())
}

fn exec(command: String, args: Vec<String>, env_variables: HashMap<String, String>) {
    info!("The following environment variables will be exposed {:?}", env_variables.keys());
    info!("Executing {} with args {:?}", command, args);
    Command::new(command)
        .args(args)
        .envs(env_variables)
        .spawn()
        .expect("Error spawning command")
        .wait()
        .expect("Command failed");
}
