use aws_sdk_dynamodb::{config::Builder as ConfigBuilder, Client as DynamoDBClient, Endpoint};
use aws_types::region::Region;
use cached::proc_macro::cached;
use http::Uri;

// Re-export
pub use aws_sdk_dynamodb;

async fn dynamodb_client(region: Option<&'static str>) -> DynamoDBClient {
    let config_builder = aws_config::from_env();

    let config = match region {
        Some(region) => config_builder.region(Region::new(region)),
        None => config_builder,
    }
    .load()
    .await;

    match std::env::var("TARGET").map(|target| target.eq_ignore_ascii_case("local")) {
        Ok(true) => {
            let dynamo_config = ConfigBuilder::from(&config)
                .endpoint_resolver(Endpoint::immutable(Uri::from_static(
                    "http://localhost:8000",
                )))
                .build();

            DynamoDBClient::from_conf(dynamo_config)
        }
        _ => DynamoDBClient::new(&config),
    }
}

#[cached]
pub async fn dynamodb(region: Option<&'static str>) -> DynamoDBClient {
    dynamodb_client(region).await
}
