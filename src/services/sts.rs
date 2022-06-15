use crate::services::in_region;
use aws_sdk_sts::Client as STSClient;
use tokio::sync::OnceCell;

async fn sts_client(region: Option<&'static str>) -> STSClient {
    STSClient::new(&in_region(region).await)
}

static STS: OnceCell<STSClient> = OnceCell::const_new();

pub async fn sts<'client>(region: Option<&'static str>) -> &'client STSClient {
    STS.get_or_init(|| sts_client(region)).await
}
