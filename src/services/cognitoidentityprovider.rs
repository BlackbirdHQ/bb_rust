use crate::services::in_region;
use aws_sdk_cognitoidentityprovider::Client as CognitoIDPClient;
use cached::proc_macro::cached;

// Re-export
pub use aws_sdk_cognitoidentityprovider;

#[cached]
pub async fn cognito(region: Option<&'static str>) -> CognitoIDPClient {
    CognitoIDPClient::new(&in_region(region).await)
}
