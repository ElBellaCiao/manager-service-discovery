use crate::model::discoverability_request::{GetAssignmentRequest, PutAssignmentRequest};
use crate::model::InstanceAssignment;
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    pub table_client: Arc<dyn cloud_util::Table<InstanceAssignment>>,
    pub instance_client: Arc<dyn cloud_util::Instance>,
}

pub async fn get_assignment(req: GetAssignmentRequest, deps: Deps) -> Result<InstanceAssignment> {
    let Deps { table_client, instance_client } = deps;

    let tags = instance_client.get_tags_by_instance(req.instance_id.as_ref()).await?;
    let group = tags.get("App")
        .ok_or_else(|| anyhow!("Instance tag 'App' does not exist"))?;
    
    let instance_assignment = table_client.get_entry(group, req.instance_id.as_ref()).await?;

    if Utc::now() > instance_assignment.expire_at {
        bail!("Assignment has expired: current time = {}, expires at = {}", Utc::now(), instance_assignment.expire_at);
    }

    Ok(instance_assignment)
}

pub async fn put_assignment(req: PutAssignmentRequest, deps: Deps) -> Result<()> {
    let Deps { table_client, instance_client } = deps;

    let tags = instance_client.get_tags_by_instance(req.instance_id.as_ref()).await?;
    let group = tags.get("App")
        .ok_or_else(|| anyhow!("Instance tag 'App' does not exist"))?
        .to_string();
    
    let metadata = instance_client.get_instance_metadata(req.instance_id.as_ref()).await?;

    let assignment = InstanceAssignment {
        group: group.parse()?,
        ip: metadata.private_ip,
        instance_id: req.instance_id,
        stocks: req.stocks,
        input: req.input,
        output: req.output,
        expire_at: req.expire_at,
    };

    table_client.put_entry(assignment).await?;
    Ok(())
}