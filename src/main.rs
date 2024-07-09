use aws_sdk_ssm::Client;
use clap::Parser;
use env_logger::Builder;
use log::debug;

use cli::Cli;

use crate::cli::SubCommand;
use crate::commands::{command_exec, command_exec_ansible_vault_mode};
use crate::errors::CliError;
use crate::ssm::{fetch_all_ssm_parameters, fetch_ssm_parameter, fetch_ssm_parameters};

mod cli;
mod commands;
mod errors;
mod ssm;

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
        SubCommand::Exec {
            ssm_path_prefix,
            contexts,
            command,
            args,
        } => {
            let env_variables =  match contexts.len() {
                0 => fetch_ssm_parameters(&ssm_client, ssm_path_prefix).await?,
                _ => fetch_all_ssm_parameters(&ssm_client, contexts.iter().map(|context| "/app/ssm-env/".to_owned() + context + "/").collect()).await?
            };
            command_exec(command, args, env_variables)?
        }
        SubCommand::ExecAnsibleVaultMode {
            ssm_path,
            command,
            args,
        } => {
            let secret = fetch_ssm_parameter(&ssm_client, ssm_path).await?;
            command_exec_ansible_vault_mode(command, args, secret)?
        }
    };
    Ok(Ok(()))
}
