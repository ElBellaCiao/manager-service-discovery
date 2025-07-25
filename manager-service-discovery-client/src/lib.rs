mod config;
mod model;

use anyhow::Result;
use cloud_util::{Api, Ec2MetadataClient, Metadata, get_config};
use config::Config;
pub use model::*;
use reqwest::Method;
use tokio::runtime::Runtime;

// Todo: get 'Group' via ssm param store
pub fn get_assignment() -> Result<AddressBook> {
    Runtime::new()?.block_on(async {
        // config
        let config = get_config::<Config>()?;

        // create request
        let metadata_client = Ec2MetadataClient::builder().build().await?;
        let group = metadata_client.get_tag_value("Group").await?;
        let ip = metadata_client.get_self_id().await?;
        let request = GetAssignmentRequest {
            group,
            ip: ip.parse()?,
        };

        // send request
        let api_client = cloud_util::RestApi::builder().build().await;
        api_client
            .send_request(
                Method::GET,
                &config.manager_service_discovery_url,
                Some(request),
            )
            .await
    })
}
