#[cfg(feature = "services_cloudformation")]
pub mod cloudformation;
#[cfg(feature = "services_cognitoidentityprovider")]
pub mod cognitoidentityprovider;
#[cfg(feature = "services_dynamodb")]
pub mod dynamodb;
#[cfg(feature = "services_organizations")]
pub mod organizations;
#[cfg(feature = "services_s3")]
pub mod s3;
#[cfg(feature = "services_secretsmanager")]
pub mod secretsmanager;
#[cfg(feature = "services_ssm")]
pub mod ssm;
#[cfg(feature = "services_sts")]
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
