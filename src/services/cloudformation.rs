use aws_sdk_cloudformation::Client as CloudformationClient;

use tokio::sync::OnceCell;

async fn cloudformation_client() -> CloudformationClient {
    let config = aws_config::from_env().region("eu-west-1").load().await;
    CloudformationClient::new(&config)
}

static COGNITO: OnceCell<CloudformationClient> = OnceCell::const_new();

pub async fn cloudformation<'client>() -> &'client CloudformationClient {
    COGNITO.get_or_init(cloudformation_client).await
}
