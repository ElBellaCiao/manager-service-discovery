use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::model::group::Group;

#[derive(Deserialize, Serialize)]
pub struct PutAssignmentBody {
    pub stocks: Vec<String>,
    pub input: Group,
    pub output: Group,
    pub expire_at: DateTime<Utc>,
}