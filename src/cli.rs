use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser)]
/// Retrieve parameters from AWS Systems Manager Parameter Store and expose them as environment variables
pub struct Cli {
    /// Configure the log level of logs
    #[arg(short, long, default_value = "INFO")]
    pub log_level: LevelFilter,

    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Executes a subcommand in a new process with environment variables populated from AWS SSM Parameter Store
    Exec {
        /// The SSM path prefix from which to retrieve the parameters
        #[arg(short, long, default_value = "/app/ssm-env/env/")]
        ssm_path_prefix: String,
        /// The contexts to retrieve before executing the command, for each context all the ssm entries from
        /// `/app/ssm-env/<context>/` will be added as environment variables
        #[arg(short, long, conflicts_with = "ssm_path_prefix")]
        contexts: Vec<String>,
        /// The command to execute
        #[arg()]
        command: String,
        /// The args to pass to the command
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Executes a subcommand with ansible-vault compatibility by creating a temporary file containing the secret and exposing the DEFAULT_VAULT_PASSWORD_FILE environment variable
    ExecAnsibleVaultMode {
        /// The SSM path from which to retrieve the ansible-vault secret
        #[arg(short, long, default_value = "/app/ssm-env/ansible-vault")]
        ssm_path: String,
        /// The command to execute
        #[arg()]
        command: String,
        /// The args to pass to the command
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}
