use crate::errors::CliError;
use log::info;
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, ExitStatus};
use tempfile::NamedTempFile;

pub fn command_exec_ansible_vault_mode(
    command: String,
    args: Vec<String>,
    secret: Option<String>,
) -> Result<ExitStatus, CliError> {
    let mut vault_file = NamedTempFile::new()?;
    let vault_file_name = &vault_file
        .path()
        .to_str()
        .unwrap_or("ssm-env-default")
        .to_string();
    info!("Temporary file {:?} created", &vault_file_name);
    vault_file.write_all(
        secret
            .ok_or(CliError::from("The provided secret is empty"))?
            .as_bytes(),
    )?;
    let env_variables: HashMap<String, String> = [(
        "ANSIBLE_VAULT_PASSWORD_FILE".to_string(),
        vault_file_name.to_string(),
    )]
    .iter()
    .cloned()
    .collect();
    let res = command_exec(command, args, env_variables, None)?;
    vault_file.close()?;
    info!("Temporary file {:?} cleaned", &vault_file_name);
    Ok(res)
}

pub fn command_exec(
    command: String,
    args: Vec<String>,
    env_variables: HashMap<String, String>,
    env_prefix: Option<String>,
) -> Result<ExitStatus, CliError> {
    let prefixed_env_variables = match env_prefix {
        Some(prefix) => env_variables
            .iter()
            .map(|(k, v)| (format!("{prefix}{k}"), v.to_string()))
            .collect(),
        None => env_variables,
    };
    info!(
        "The following environment variables will be exposed {:?}",
        prefixed_env_variables.keys()
    );
    info!("Executing {} with args {:?}", command, args);
    let mut sub = Command::new(command)
        .args(args)
        .envs(prefixed_env_variables)
        .spawn()?;

    sub.wait().map_err(|e| e.into())
}
