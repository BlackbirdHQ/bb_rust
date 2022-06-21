use crate::services::in_region;
use aws_sdk_apigateway::Client as ApiGatewayClient;
use tokio::sync::OnceCell;

// Re-export
pub use aws_sdk_cloudformation;

async fn apigateway_client(region: Option<&'static str>) -> ApiGatewayClient {
    ApiGatewayClient::new(&in_region(region).await)
}

static APIGATEWAY: OnceCell<ApiGatewayClient> = OnceCell::const_new();

pub async fn apigateway<'client>(region: Option<&'static str>) -> &'client ApiGatewayClient {
    APIGATEWAY.get_or_init(|| apigateway_client(region)).await
}
