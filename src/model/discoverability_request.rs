use crate::model::instance_id::InstanceId;
use crate::model::group::Group;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug)]
pub struct GetAssignmentRequest {
    pub instance_id: InstanceId,
}

#[derive(Debug)]
pub struct PutAssignmentRequest {
    pub instance_id: InstanceId,
    pub stocks: Vec<String>,
    pub input: Group,
    pub output: Group,
    pub expire_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct PutAssignmentRequestBody {
    pub stocks: Vec<String>,
    pub input: Group,
    pub output: Group,
    pub expire_at: DateTime<Utc>,
}
