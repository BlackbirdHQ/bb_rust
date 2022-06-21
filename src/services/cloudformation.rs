use crate::services::in_region;
use aws_sdk_cloudformation::Client as CloudformationClient;
use tokio::sync::OnceCell;

// Re-export
pub use aws_sdk_cloudformation;

async fn cloudformation_client(region: Option<&'static str>) -> CloudformationClient {
    CloudformationClient::new(&in_region(region).await)
}

static COGNITO: OnceCell<CloudformationClient> = OnceCell::const_new();

pub async fn cloudformation<'client>(
    region: Option<&'static str>,
) -> &'client CloudformationClient {
    COGNITO.get_or_init(|| cloudformation_client(region)).await
}
