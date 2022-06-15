use crate::services::in_region;
use aws_sdk_secretsmanager::Client as SecretsManagerClient;
use tokio::sync::OnceCell;

async fn secrets_manager_client(region: Option<&'static str>) -> SecretsManagerClient {
    SecretsManagerClient::new(&in_region(region).await)
}

static SECRETS_MANAGER: OnceCell<SecretsManagerClient> = OnceCell::const_new();

pub async fn secrets_manager<'client>(
    region: Option<&'static str>,
) -> &'client SecretsManagerClient {
    SECRETS_MANAGER
        .get_or_init(|| secrets_manager_client(region))
        .await
}
