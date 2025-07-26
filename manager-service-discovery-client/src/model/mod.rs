use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    // todo: Grab instrument id Type from a common library
    pub instrument_ids: Vec<u32>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AddressBook {
    // todo: Grab instrument id Type from a common library
    pub instrument_to_ips: HashMap<u32, Vec<IpAddr>>,
}
