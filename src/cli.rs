use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser)]
/// Retrieve parameters from AWS Systems Manager Parameter Store and expose them as environment variables
#[command(version)]
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
    /// If neither ssm_path_prefixes nor contexts are provided, values will be retrieved from the "/app/ssm-env/env/<ENV_VARIABLES>" path.
    Exec {
        /// The SSM path prefixes from which to retrieve the parameters.
        #[arg(short, long)]
        ssm_path_prefixes: Vec<String>,
        /// When using the contexts argument, environment variables coming from "/app/ssm-env/env/{context}/<ENV_VARIABLES>" will be injected.
        #[arg(short, long)]
        contexts: Vec<String>,
        /// Will prefix the environment variables with the given prefix
        #[arg(short, long)]
        env_prefix: Option<String>,
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
