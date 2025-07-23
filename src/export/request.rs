use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Deserialize, Serialize)]
pub struct PutAssignmentRequest {
    pub assignments: Vec<Assignment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAssignmentRequest {
    pub group: String,
    pub ip: IpAddr,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Assignment {
    pub group: String,
    pub ip: IpAddr,
    pub instrument_ids: Vec<usize>,
    pub output_groups: Vec<String>,

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
        self.ip.to_string()
    }
}
