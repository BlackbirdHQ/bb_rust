use crate::services::in_region;
use aws_sdk_cognitoidentityprovider::Client as CognitoIDPClient;
use tokio::sync::OnceCell;

// Re-export
pub use aws_sdk_cognitoidentityprovider;

async fn cognito_client(region: Option<&'static str>) -> CognitoIDPClient {
    CognitoIDPClient::new(&in_region(region).await)
}

static COGNITO: OnceCell<CognitoIDPClient> = OnceCell::const_new();

pub async fn cognito<'client>(region: Option<&'static str>) -> &'client CognitoIDPClient {
    COGNITO.get_or_init(|| cognito_client(region)).await
}
