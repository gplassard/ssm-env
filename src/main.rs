use std::process::Command;
use aws_sdk_ssm as ssm;
use std::collections::HashMap;

#[::tokio::main]
async fn main() -> Result<(), ssm::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_ssm::Client::new(&config);

    let result = client.get_parameters_by_path().path("/").recursive(true).send().await?;

    let env_variables: HashMap<String, String> = result.parameters().iter()
        .filter_map(|parameter| {
            match (parameter.name(), parameter.value()) {
                (Some(name), Some(value)) => Some((name.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect();

    Command::new("powershell")
        .arg("Get-ChildItem Env:")
        .envs(env_variables)
        .spawn()
        .expect("Error")
        .wait()
        .expect("Failed");
    Ok(())
}
