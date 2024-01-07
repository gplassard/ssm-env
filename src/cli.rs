use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser)]
/// Retrieve parameters from AWS Systems Manager Parameter Store and expose them as environment variables
pub struct Cli {
    /// Configure the log level of logs
    #[structopt(short, long, default_value="INFO")]
    pub log_level: LevelFilter,

    /// The SSM path from which to retrieve the parameters
    #[structopt(short, long, default_value="/")]
    pub path: String,

    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Executes a subcommand in a new process with environment variables populated from AWS SSM Parameter Store
    Exec {
        /// The command to execute
        #[arg()]
        command: String,
        /// The args to pass to the command
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}
