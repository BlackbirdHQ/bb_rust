use aws_sdk_secretsmanager::Client as SecretsManagerClient;
use aws_types::region::Region;
use tokio::sync::OnceCell;

async fn secrets_manager_client() -> SecretsManagerClient {
    let config = aws_config::from_env()
        .region(Region::new("eu-west-1"))
        .load()
        .await;
    SecretsManagerClient::new(&config)
}

static SECRETS_MANAGER: OnceCell<SecretsManagerClient> = OnceCell::const_new();

pub async fn secrets_manager<'client>() -> &'client SecretsManagerClient {
    SECRETS_MANAGER.get_or_init(secrets_manager_client).await
}
