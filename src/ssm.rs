use aws_sdk_ssm::Client;
use aws_sdk_ssm::Error as SsmError;
use log::debug;
use std::collections::HashMap;

pub async fn fetch_ssm_parameters(
    ssm_client: Client,
    path_prefix: String,
) -> Result<HashMap<String, String>, SsmError> {
    debug!("retrieving SSM parameters from path {}", &path_prefix);
    let result = ssm_client
        .get_parameters_by_path()
        .path(&path_prefix)
        .send()
        .await?;
    debug!("SSM parameters retrieved");

    let env_variables: HashMap<String, String> = result
        .parameters()
        .iter()
        .filter_map(|parameter| match (parameter.name(), parameter.value()) {
            (Some(name), Some(value)) => Some((
                name.to_string().replacen(&path_prefix, "", 1),
                value.to_string(),
            )),
            _ => None,
        })
        .collect();
    Ok(env_variables)
}
pub async fn fetch_ssm_parameter(
    ssm_client: Client,
    path: String,
) -> Result<Option<String>, SsmError> {
    debug!("retrieving SSM parameter {}", &path);
    let result = ssm_client.get_parameter().name(&path).send().await?;
    debug!("SSM parameter retrieved");

    Ok(result.parameter.and_then(|p| p.value))
}
