use clap::{Parser, Subcommand};

#[derive(Parser)]
/// Retrieve parameters from AWS Systems Manager Parameter Store and expose them as environment variables
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    Exec {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        args: Vec<String>,
    }
}
