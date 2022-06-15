pub mod cloudformation;
pub mod cognito;
pub mod dynamodb;
pub mod s3;
pub mod secrets_manager;
pub mod ssm;
pub mod sts;

use aws_types::region::Region;
use aws_types::SdkConfig;

pub async fn in_region(region: Option<&'static str>) -> SdkConfig {
    let config_builder = aws_config::from_env();

    match region {
        Some(region) => config_builder.region(Region::new(region)),
        None => config_builder,
    }
    .load()
    .await
}
