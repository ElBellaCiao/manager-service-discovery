use crate::resource::assignment_client::AssignmentsClient;

#[derive(Debug, Clone)]
pub struct ServiceDiscoveryClient {
    rest_client: cloud_util::RestApi,
}

impl ServiceDiscoveryClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            rest_client: cloud_util::RestApi::new(base_url.into())
        }
    }

    pub fn assignments(&self) -> AssignmentsClient {
        AssignmentsClient::new(self)
    }
    
    pub(crate) fn get_rest_client(&self) -> &cloud_util::RestApi {
        &self.rest_client
    }
}
