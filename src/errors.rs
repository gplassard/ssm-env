use aws_sdk_ssm::Error as SsmError;

#[derive(Debug)]
#[allow(dead_code)]
pub struct CliError {
    message: String,
    error: Option<Box<dyn std::error::Error>>,
}


impl From<SsmError> for CliError {
    fn from(error: SsmError) -> Self {
        CliError {
            message: "AWS SSM error".to_string(),
            error: Some(Box::new(error)),
        }
    }
}
impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError {
            message: "IO error".to_string(),
            error: Some(Box::new(error)),
        }
    }
}
impl From<&str> for CliError {
    fn from(message: &str) -> Self {
        CliError {
            message: message.to_string(),
            error: None,
        }
    }
}