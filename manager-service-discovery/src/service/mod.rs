use anyhow::{Result, bail};
use chrono::Utc;
use lambda_http::Request;
use manager_service_discovery_client::{
    AddressBook, Assignment, GetAssignmentRequest, PutAssignmentRequest,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ServiceDiscovery {
    table_client: Arc<dyn cloud_util::Table<Assignment>>,
}

impl ServiceDiscovery {
    pub fn new(table_client: Arc<dyn cloud_util::Table<Assignment>>) -> Self {
        Self { table_client }
    }

    pub async fn put_assignment(&self, req: Request) -> Result<()> {
        let put_request = serde_json::from_slice::<PutAssignmentRequest>(req.body().as_ref())?;
        for assignment in put_request.assignments {
            self.table_client.put_entry(assignment).await?;
        }

        Ok(())
    }

    pub async fn get_assignment(&self, req: Request) -> Result<AddressBook> {
        let get_request = serde_json::from_slice::<GetAssignmentRequest>(req.body().as_ref())?;
        let assignment = self
            .table_client
            .get_entry(&get_request.group, &get_request.ip.to_string())
            .await?;

        if Utc::now() > assignment.expire_at {
            bail!(
                "Assignment has expired: current time = {}, expires at = {}",
                Utc::now(),
                assignment.expire_at
            );
        }

        let mut instrument_to_ips = HashMap::new();

        for output_group in assignment.output_groups {
            let group_assignments = self.table_client.get_entries_by_pk(&output_group).await?;
            for output_assignment in group_assignments {
                for instrument_id in output_assignment.instrument_ids {
                    if !assignment.instrument_ids.contains(&instrument_id) {
                        continue;
                    }
                    instrument_to_ips
                        .entry(instrument_id.to_string())
                        .or_insert_with(Vec::new)
                        .push(output_assignment.ip);
                }
            }
        }

        Ok(AddressBook { instrument_to_ips })
    }
}
