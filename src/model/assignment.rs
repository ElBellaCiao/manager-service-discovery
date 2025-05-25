use crate::model::Group;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use cloud_util::InstanceId;

#[derive(Debug, Deserialize, Serialize)]
pub struct Assignment {
    pub group: Group,
    pub instance_id: InstanceId,
    pub stocks: Vec<String>,
    pub input: Group,
    pub output: Group,
    pub ip: IpAddr,

    // format required by ddb
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expire_at: DateTime<Utc>,

    // Consider adding a status field.
}

impl cloud_util::Keyed for Assignment {
    fn pk(&self) -> String {
        self.group.to_string()
    }

    fn sk(&self) -> String {
        self.instance_id.to_string()
    }
}
