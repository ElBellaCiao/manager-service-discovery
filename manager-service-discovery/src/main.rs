use crate::common::{error_response, success_response};
use crate::config::{Deps, Settings};
use crate::service::ServiceDiscovery;
use anyhow::Result;
use aws_lambda_events::http::Method;
use lambda_http::{Body, Request, Response, run, service_fn};
use serde_json::json;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod common;
mod config;
mod service;

async fn handler(req: Request, deps: Deps) -> Result<Response<Body>, lambda_http::Error> {
    info!("Request: {:?}", req);

    let response = match *req.method() {
        Method::GET => deps.service_discovery.get_assignment(req).await,
        Method::PUT => deps.service_discovery.put_assignment(req).await,
        _ => Ok(json!(format!("Method Not Allowed: {}", *req.method()))),
    };

    match response {
        Ok(result) => Ok(success_response(result)),
        Err(e) => Ok(error_response(500, e)),
    }
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time() // Cloudwatch adds timestamp already
        .json()
        .init();

    let settings = Settings::load_config()?;
    let table_client = Arc::new(
        cloud_util::DynamoDbClient::builder()
            .table_name(&settings.table_name)
            .build()
            .await?,
    );

    let deps = Deps {
        service_discovery: Arc::new(ServiceDiscovery::new(table_client)),
    };

    run(service_fn(move |req| handler(req, deps.clone()))).await
}
