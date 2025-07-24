use crate::service::ServiceDiscovery;
use anyhow::Result;
use config::{Config, Environment};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    pub service_discovery: Arc<ServiceDiscovery>,
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub table_name: String,
}

impl Settings {
    pub fn load_config() -> Result<Self> {
        let settings = Config::builder()
            .add_source(Environment::default())
            .build()?
            .try_deserialize()?;
        Ok(settings)
    }
}
