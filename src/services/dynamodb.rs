use aws_sdk_dynamodb::{config::Builder as ConfigBuilder, Client as DynamoDBClient, Endpoint};
use http::Uri;
use tokio::sync::OnceCell;

async fn dynamodb_client() -> DynamoDBClient {
    let config = aws_config::load_from_env().await;

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

pub async fn dynamodb<'client>() -> &'client DynamoDBClient {
    CLIENT.get_or_init(dynamodb_client).await
}
