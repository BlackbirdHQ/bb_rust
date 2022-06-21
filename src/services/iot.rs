use crate::services::in_region;
use aws_sdk_iot::Client as IotClient;
use tokio::sync::OnceCell;

// Re-export
pub use aws_sdk_iot;

async fn iot_client(region: Option<&'static str>) -> IotClient {
    IotClient::new(&in_region(region).await)
}

static IOT: OnceCell<IotClient> = OnceCell::const_new();

pub async fn iot<'client>(region: Option<&'static str>) -> &'client IotClient {
    IOT.get_or_init(|| iot_client(region)).await
}
