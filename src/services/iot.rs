use crate::services::in_region;
use aws_sdk_iot::Client as IotClient;
use cached::proc_macro::cached;

// Re-export
pub use aws_sdk_iot;

#[cached]
pub async fn iot(region: Option<&'static str>) -> IotClient {
    IotClient::new(&in_region(region).await)
}
