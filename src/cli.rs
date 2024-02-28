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
        /// The SSM path prefix from which to retrieve the parameters.
        /// Cannot be used in conjunction with the context argument
        #[arg(
            short,
            long,
            default_value = "/app/ssm-env/env/",
            conflicts_with = "context"
        )]
        ssm_path_prefix: String,
        /// When using the context argument, environment variables will be retrieved from "/app/ssm-env/env/{context}/" instead.
        /// Cannot be used in conjunction with the ssm_path_prefix argument
        #[arg(short, long, conflicts_with = "ssm_path_prefix")]
        context: Option<String>,
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
