use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Group {
    Input,
    Parser,
    Lob,
    Ml,
    Orchestrator,
    Executor,
    Output,
}