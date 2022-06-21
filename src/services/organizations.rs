use crate::services::in_region;
use aws_sdk_organizations::Client as OrganizationsClient;
use tokio::sync::OnceCell;

// Re-export
pub use aws_sdk_organizations;

async fn organizations_client(region: Option<&'static str>) -> OrganizationsClient {
    OrganizationsClient::new(&in_region(region).await)
}

static ORGANIZATIONS: OnceCell<OrganizationsClient> = OnceCell::const_new();

pub async fn organizations<'client>(region: Option<&'static str>) -> &'client OrganizationsClient {
    ORGANIZATIONS
        .get_or_init(|| organizations_client(region))
        .await
}
