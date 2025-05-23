use crate::model::group::Group;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use cloud_util::InstanceId;

#[derive(Debug, Deserialize, Serialize)]
pub struct InstanceAssignment {
    // Name required by ddb
    #[serde(rename = "PK")]
    pub group: Group,

    // Name required by ddb
    #[serde(rename = "SK")]
    pub instance_id: InstanceId,

    // format required by ddb
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expire_at: DateTime<Utc>,

    pub stocks: Vec<String>,
    pub input: Group,
    pub output: Group,
    pub ip: IpAddr,

    // Consider adding a status field.
}

impl cloud_util::Keyed for InstanceAssignment {
    fn pk(&self) -> String {
        self.group.to_string()
    }

    fn sk(&self) -> String {
        self.instance_id.to_string()
    }
}
