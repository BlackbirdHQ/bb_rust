use aws_sdk_cognitoidentityprovider::Client as CognitoIDPClient;
use aws_types::region::Region;

use tokio::sync::OnceCell;

async fn cognito_client() -> CognitoIDPClient {
    let config = aws_config::load_from_env().await;
    CognitoIDPClient::new(&config)
}

static COGNITO: OnceCell<CognitoIDPClient> = OnceCell::const_new();

pub async fn cognito<'client>() -> &'client CognitoIDPClient {
    COGNITO.get_or_init(cognito_client).await
}
