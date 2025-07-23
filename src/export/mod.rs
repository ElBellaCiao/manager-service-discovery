mod request;
mod service_discovery_client;

pub use service_discovery_client::ServiceDiscoveryClient;

pub use request::{Assignment, GetAssignmentRequest, PutAssignmentRequest};
