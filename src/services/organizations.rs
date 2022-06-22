use crate::services::in_region;
use aws_sdk_organizations::Client as OrganizationsClient;

// Re-export
pub use aws_sdk_organizations;
use cached::proc_macro::cached;

#[cached]
pub async fn organizations(region: Option<&'static str>) -> OrganizationsClient {
    OrganizationsClient::new(&in_region(region).await)
}
