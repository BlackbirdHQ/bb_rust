use aws_sdk_s3::Client as S3Client;
use aws_types::region::Region;
use tokio::sync::OnceCell;

async fn s3_client() -> S3Client {
    let config = aws_config::from_env()
        .region(Region::new("eu-west-1"))
        .load()
        .await;
    S3Client::new(&config)
}

static S3: OnceCell<S3Client> = OnceCell::const_new();

pub async fn s3<'client>() -> &'client S3Client {
    S3.get_or_init(s3_client).await
}
