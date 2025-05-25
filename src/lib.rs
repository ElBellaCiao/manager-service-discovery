mod resource;
mod model;

pub use cloud_util::InstanceId;

pub mod client {
    pub use crate::resource::ServiceDiscoveryClient;
}
pub mod request {
    pub use crate::model::request::*;
}

pub mod types {
    pub use crate::model::{Group, Assignment};
}
