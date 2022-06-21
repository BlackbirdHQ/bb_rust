#[cfg(feature = "graphql")]
pub mod graphql;
#[cfg(feature = "misc")]
pub mod misc;
#[cfg(any(
    feature = "services_apigateway",
    feature = "services_cloudformation",
    feature = "services_cognitoidentityprovider",
    feature = "services_dynamodb",
    feature = "services_lambda",
    feature = "services_organizations",
    feature = "services_s3",
    feature = "services_secretsmanager",
    feature = "services_ssm",
    feature = "services_sts",
))]
pub mod services;
#[cfg(feature = "types")]
pub mod types;
