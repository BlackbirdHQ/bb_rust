use crate::services::in_region;
use aws_sdk_ssm::Client as SSMClient;
use tokio::sync::OnceCell;

async fn ssm_client(region: Option<&'static str>) -> SSMClient {
    SSMClient::new(&in_region(region).await)
}

static SSM: OnceCell<SSMClient> = OnceCell::const_new();

pub async fn ssm<'client>(region: Option<&'static str>) -> &'client SSMClient {
    SSM.get_or_init(|| ssm_client(region)).await
}
