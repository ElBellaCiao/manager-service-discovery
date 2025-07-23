use anyhow::{Result, bail};
use chrono::Utc;
use lambda_http::Request;
use serde_json::{Value, json};
use service_discovery::{Assignment, GetAssignmentRequest, PutAssignmentRequest};
use std::sync::Arc;

pub struct ServiceDiscovery {
    table_client: Arc<dyn cloud_util::Table<Assignment>>,
}

impl ServiceDiscovery {
    pub fn new(table_client: Arc<dyn cloud_util::Table<Assignment>>) -> Self {
        Self { table_client }
    }

    pub async fn put_assignment(&self, req: Request) -> Result<Value> {
        let put_request = serde_json::from_slice::<PutAssignmentRequest>(req.body().as_ref())?;
        for assignment in put_request.assignments {
            self.table_client.put_entry(assignment).await?;
        }

        Ok(json!("Success"))
    }

    pub async fn get_assignment(&self, req: Request) -> Result<Value> {
        let get_request = serde_json::from_slice::<GetAssignmentRequest>(req.body().as_ref())?;
        let instance_assignment = self
            .table_client
            .get_entry(&get_request.group, &get_request.ip.to_string())
            .await?;
        if Utc::now() > instance_assignment.expire_at {
            bail!(
                "Assignment has expired: current time = {}, expires at = {}",
                Utc::now(),
                instance_assignment.expire_at
            );
        }

        Ok(json!(instance_assignment))
    }
}
