use aws_sdk_ssm::Client as SSMClient;
use aws_types::region::Region;
use tokio::sync::OnceCell;

async fn ssm_client() -> SSMClient {
    let config = aws_config::from_env()
        .region(Region::new("eu-west-1"))
        .load()
        .await;
    SSMClient::new(&config)
}

static SSM: OnceCell<SSMClient> = OnceCell::const_new();

pub async fn ssm<'client>() -> &'client SSMClient {
    SSM.get_or_init(ssm_client).await
}
