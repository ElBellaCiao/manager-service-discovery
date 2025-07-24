use crate::{Assignment, GetAssignmentRequest};
use anyhow::Result;
use cloud_util::Api;
use reqwest::Method;

pub struct ServiceDiscoveryClient {
    client: cloud_util::RestApi,
    base_url: String,
}

impl ServiceDiscoveryClient {
    pub fn new(client: cloud_util::RestApi, base_url: &str) -> Self {
        Self {
            client,
            base_url: base_url.to_owned(),
        }
    }

    pub async fn get(&self, request: GetAssignmentRequest) -> Result<Assignment> {
        let response = self
            .client
            .send_request(Method::GET, &self.base_url, Some(request))
            .await?;
        Ok(response)
    }
}
