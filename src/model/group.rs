use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Group {
    Parser,
    Lob,
    Ml,
    Orchestrator,
    Executor
}