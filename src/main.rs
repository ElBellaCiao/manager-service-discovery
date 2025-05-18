use crate::common::Config;
use crate::service::discoverability_service::Deps;
use anyhow::Result;
use aws_lambda_events::http::Method;
use lambda_http::{run, service_fn, Body, Request, Response};
use std::sync::Arc;

mod routes;
mod model;
mod service;
mod common;

async fn handler(req: Request, deps: Deps) -> Result<Response<Body>, lambda_http::Error> {
    match *req.method() {
        Method::GET => Ok(routes::discoverability_routes::handle_get(req, deps).await),
        Method::PUT => Ok(routes::discoverability_routes::handle_put(req, deps).await),
        _ => Ok(Response::builder()
            .status(405)
            .body("Method Not Allowed".into())?),
    }
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    let config = Config::load_from_env();
    let deps = Deps {
        instance_client: Arc::new(cloud_util::Ec2::new(None).await),
        table_client: Arc::new(cloud_util::DynamoDb::new(None, config.table_name.clone()).await)
    };
    
    run(service_fn(move |req| handler(req, deps.clone()))).await
}