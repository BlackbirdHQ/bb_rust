use crate::services::in_region;
use aws_sdk_secretsmanager::Client as SecretsManagerClient;
use cached::proc_macro::cached;

// Re-export
pub use aws_sdk_secretsmanager;

#[cached]
pub async fn secrets_manager(region: Option<&'static str>) -> SecretsManagerClient {
    SecretsManagerClient::new(&in_region(region).await)
}
