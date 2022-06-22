use crate::services::in_region;
use aws_sdk_ssm::Client as SSMClient;
use cached::proc_macro::cached;

// Re-export
pub use aws_sdk_ssm;

#[cached]
pub async fn ssm(region: Option<&'static str>) -> SSMClient {
    SSMClient::new(&in_region(region).await)
}
