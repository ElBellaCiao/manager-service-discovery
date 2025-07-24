mod config;
mod model;
mod service;

use crate::service::ServiceDiscoveryClient;
use cloud_util::{Ec2MetadataClient, Metadata, get_config};
use config::Config;
pub use model::*;

pub async fn get_assignment() -> anyhow::Result<Assignment> {
    let config = get_config::<Config>()?;
    let api_client = cloud_util::RestApi::builder().build().await;
    let service_discovery_client =
        ServiceDiscoveryClient::new(api_client, &config.manager_service_discovery_url);

    let metadata_client = Ec2MetadataClient::builder().build().await?;
    let group = metadata_client.get_tag_value("App").await?;
    let ip = metadata_client.get_self_id().await?;

    let request = GetAssignmentRequest {
        group,
        ip: ip.parse()?,
    };

    service_discovery_client.get(request).await
}
