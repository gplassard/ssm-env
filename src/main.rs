use aws_sdk_ssm::Client;
use clap::Parser;
use env_logger::Builder;
use log::debug;

use cli::Cli;

use crate::cli::SubCommand;
use crate::commands::{command_exec, command_exec_ansible_vault_mode};
use crate::errors::CliError;
use crate::ssm::{fetch_ssm_parameter, fetch_ssm_parameters};

mod cli;
mod errors;
mod ssm;
mod commands;

#[::tokio::main]
async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    Builder::new().filter_level(cli.log_level).init();

    run(cli).await?
}

async fn run(cli: Cli) -> Result<Result<(), CliError>, CliError> {
    debug!("retrieving AWS credentials");
    let aws_config = aws_config::load_from_env().await;
    let ssm_client = Client::new(&aws_config);


    match cli.command {
        SubCommand::Exec { ssm_path_prefix, command, args } => {
            let env_variables = fetch_ssm_parameters(ssm_client, ssm_path_prefix).await?;
            command_exec(command, args, env_variables)?
        },
        SubCommand::ExecAnsibleVaultMode { ssm_path, command, args } => {
            let secret = fetch_ssm_parameter(ssm_client, ssm_path).await?;
            command_exec_ansible_vault_mode(command, args, secret)?
        },
    };
    Ok(Ok(()))
}
