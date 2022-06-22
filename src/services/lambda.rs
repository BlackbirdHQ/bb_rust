use crate::services::in_region;
use aws_sdk_lambda::Client as LambdaClient;
use cached::proc_macro::cached;

// Re-export
pub use aws_sdk_lambda;

#[cached]
pub async fn lambda(region: Option<&'static str>) -> LambdaClient {
    LambdaClient::new(&in_region(region).await)
}
