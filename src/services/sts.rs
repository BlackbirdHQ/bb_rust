use aws_sdk_sts::Client as STSClient;
use tokio::sync::OnceCell;

async fn sts_client() -> STSClient {
    let config = aws_config::load_from_env().await;
    STSClient::new(&config)
}

static STS: OnceCell<STSClient> = OnceCell::const_new();

pub async fn sts<'client>() -> &'client STSClient {
    STS.get_or_init(sts_client).await
}
