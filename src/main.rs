use crate::common::response::error_response;
use crate::common::Config;
use crate::service::discoverability_service::Deps;
use anyhow::Result;
use aws_lambda_events::http::Method;
use lambda_http::{run, service_fn, Body, Request, Response};
use std::sync::Arc;
use tracing::{instrument, Span};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

mod routes;
mod model;
mod service;
mod common;

#[instrument(skip(deps), fields(request_id = tracing::field::Empty))]
async fn handler(req: Request, deps: Deps) -> Result<Response<Body>, lambda_http::Error> {
    let request_id = Uuid::new_v4();
    Span::current().record("request_id", tracing::field::display(&request_id));

    match *req.method() {
        Method::GET => Ok(routes::discoverability_routes::handle_get(req, deps).await),
        Method::PUT => Ok(routes::discoverability_routes::handle_put(req, deps).await),
        _ => Ok(error_response(405, format!("Method Not Allowed: {}", Method::GET))),
    }
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_level(true)
        .init();

    let config = Config::load_from_env();
    let deps = Deps {
        instance_client: Arc::new(cloud_util::Ec2::new(None).await),
        table_client: Arc::new(cloud_util::DynamoDb::new(None, config.table_name.clone()).await)
    };

    run(service_fn(move |req| handler(req, deps.clone()))).await
}