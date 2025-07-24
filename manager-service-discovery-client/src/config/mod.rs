use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub manager_service_discovery_url: String,
}
