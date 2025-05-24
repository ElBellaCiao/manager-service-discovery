use crate::model::body::PutAssignmentBody;
use crate::model::request::{GetAssignmentRequest, PutAssignmentRequest};
use crate::model::Assignment;
use crate::ServiceDiscoveryClient;
use anyhow::Result;
use cloud_util::Api;
use reqwest::Method;

pub struct AssignmentsClient<'a> {
    root: &'a ServiceDiscoveryClient,
}

impl<'a> AssignmentsClient<'a> {
    pub(crate) fn new(client: &'a ServiceDiscoveryClient) -> Self {
        Self { root: client }
    }
    
    pub async fn get(&self, request: GetAssignmentRequest) -> Result<Assignment> {
        let url_suffix = format!("service-discovery/{}", request.instance_id);
        let response = self.root.get_rest_client()
            .send_request(Method::GET, &url_suffix, None::<()>).await?;
        Ok(response)
    }
    
    pub async fn put(&self, request: PutAssignmentRequest) -> Result<Assignment> {
        let url_suffix = format!("service-discovery/{}", request.instance_id);

        let body = PutAssignmentBody {
            stocks: request.stocks,
            input: request.input,
            output: request.output,
            expire_at: request.expire_at,
        };

        let response = self.root.get_rest_client()
            .send_request(Method::PUT, &url_suffix, Some(body)).await?;
        Ok(response)
    }
}