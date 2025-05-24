use crate::model::group::Group;
use chrono::{DateTime, Utc};
use cloud_util::InstanceId;
use derive_builder::Builder;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct GetAssignmentRequest{
    #[builder(setter(into))]
    pub instance_id: InstanceId,
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct PutAssignmentRequest {
    #[builder(setter(into))]
    pub instance_id: InstanceId,

    #[builder(setter(into))]
    pub stocks: Vec<String>,

    #[builder(setter(into))]
    pub input: Group,

    #[builder(setter(into))]
    pub output: Group,

    #[builder(setter(into))]
    pub expire_at: DateTime<Utc>,
}
