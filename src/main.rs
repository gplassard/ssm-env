use std::collections::HashMap;
use std::process::Command;

use aws_sdk_ssm as ssm;

use cli::Cli;
use crate::cli::SubCommand;
use clap::Parser;

mod cli;

#[::tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<(), ssm::Error> {
    let cli = Cli::parse();
    let aws_config = aws_config::load_from_env().await;
    let ssm_client = aws_sdk_ssm::Client::new(&aws_config);

    let result = ssm_client.get_parameters_by_path().path("/").recursive(true).send().await?;

    let env_variables: HashMap<String, String> = result.parameters().iter()
        .filter_map(|parameter| {
            match (parameter.name(), parameter.value()) {
                (Some(name), Some(value)) => Some((name.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect();

    match cli.command {
        SubCommand::Exec { args } => {
            exec(args, env_variables)
        }
    };
    Ok(())
}

fn exec(args: Vec<String>, env_variables: HashMap<String, String>) {
    Command::new(args.get(0).unwrap())
        .args(&args[1..])
        .envs(env_variables)
        .spawn()
        .expect("Error spawning command")
        .wait()
        .expect("Command failed");
}
