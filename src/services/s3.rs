use crate::services::in_region;
use aws_sdk_s3::Client as S3Client;
use cached::proc_macro::cached;

// Re-export
pub use aws_sdk_s3;

#[cached]
pub async fn s3(region: Option<&'static str>) -> S3Client {
    S3Client::new(&in_region(region).await)
}
