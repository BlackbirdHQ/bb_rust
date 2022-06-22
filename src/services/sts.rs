use crate::services::in_region;
use aws_sdk_sts::Client as STSClient;
use cached::proc_macro::cached;

// Re-export
pub use aws_sdk_sts;

#[cached]
pub async fn sts(region: Option<&'static str>) -> STSClient {
    STSClient::new(&in_region(region).await)
}
