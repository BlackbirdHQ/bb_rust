use aws_sdk_dynamodb::{config::Builder as ConfigBuilder, Client as DynamoDBClient, Endpoint};
use aws_types::region::Region;
use http::Uri;
use tokio::sync::OnceCell;

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

pub static CLIENT: OnceCell<DynamoDBClient> = OnceCell::const_new();

pub async fn dynamodb<'client>(region: Option<&'static str>) -> &'client DynamoDBClient {
    CLIENT.get_or_init(|| dynamodb_client(region)).await
}
