use crate::services::in_region;
use aws_sdk_apigateway::Client as ApiGatewayClient;
use cached::proc_macro::cached;

// Re-export
pub use aws_sdk_apigateway;

#[cached]
pub async fn apigateway<'client>(region: Option<&'static str>) -> ApiGatewayClient {
    ApiGatewayClient::new(&in_region(region).await)
}
