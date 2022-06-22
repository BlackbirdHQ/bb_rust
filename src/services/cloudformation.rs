use crate::services::in_region;
use aws_sdk_cloudformation::Client as CloudformationClient;

// Re-export
pub use aws_sdk_cloudformation;
use cached::proc_macro::cached;

#[cached]
pub async fn cloudformation(region: Option<&'static str>) -> CloudformationClient {
    CloudformationClient::new(&in_region(region).await)
}
