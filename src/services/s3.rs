use crate::services::in_region;
use aws_sdk_s3::Client as S3Client;
use tokio::sync::OnceCell;

// Re-export
pub use aws_sdk_s3;

async fn s3_client(region: Option<&'static str>) -> S3Client {
    S3Client::new(&in_region(region).await)
}

static S3: OnceCell<S3Client> = OnceCell::const_new();

pub async fn s3<'client>(region: Option<&'static str>) -> &'client S3Client {
    S3.get_or_init(|| s3_client(region)).await
}
