use crate::services::in_region;
use aws_sdk_lambda::Client as LambdaClient;
use tokio::sync::OnceCell;

// Re-export
pub use aws_sdk_lambda;

async fn lambda_client(region: Option<&'static str>) -> LambdaClient {
    LambdaClient::new(&in_region(region).await)
}

static LAMBDA: OnceCell<LambdaClient> = OnceCell::const_new();

pub async fn lambda<'client>(region: Option<&'static str>) -> &'client LambdaClient {
    LAMBDA.get_or_init(|| lambda_client(region)).await
}
