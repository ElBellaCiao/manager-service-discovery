use anyhow::{bail, Result};
use cloud_util::InstanceId;
use crate::common::response::{error_response, success_response};
use crate::model::discoverability_request::{GetAssignmentRequest, PutAssignmentRequest, PutAssignmentRequestBody};
use crate::service::discoverability_service;
use crate::service::discoverability_service::Deps;
use lambda_http::{Body, Request, RequestExt, Response};
use tracing::{error, instrument, warn, Span};

#[instrument(skip(deps), fields(instance_id = tracing::field::Empty))]
pub async fn handle_get(req: Request, deps: Deps) -> Response<Body> {
    let instance_id = match parse_instance_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(400, e.to_string())
    };

    Span::current().record("instance_id", tracing::field::display(&instance_id));

    let get_assignment_request = GetAssignmentRequest { instance_id };

    let result = match discoverability_service::get_assignment(get_assignment_request, deps).await {
        Ok(val) => val,
        Err(e) => {
            error!(error = ?e, "Failed to fetch instance assignment");
            return error_response(500, "Internal server error");
        }
    };

    match serde_json::to_string(&result) {
        Ok(body) => success_response(Some(body.into())),
        Err(e) => {
            error!(error = ?e, "Failed to serialize response body");
            error_response(500, "Internal server error")
        }
    }
}

#[instrument(skip(deps), fields(instance_id = tracing::field::Empty))]
pub async fn handle_put(req: Request, deps: Deps) -> Response<Body> {
    let instance_id = match parse_instance_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(400, e.to_string())
    };

    Span::current().record("instance_id", tracing::field::display(&instance_id));

    let body: PutAssignmentRequestBody = match serde_json::from_slice(req.body().as_ref()) {
        Ok(b) => b,
        Err(e) => {
            warn!(error = ?e, "Failed to parse request body as JSON");
            return error_response(400, "Invalid request body");
        }
    };

    let put_request = PutAssignmentRequest {
        instance_id,
        stocks: body.stocks,
        input: body.input,
        output: body.output,
        expire_at: body.expire_at,
    };

    if let Err(e) = discoverability_service::put_assignment(put_request, deps).await {
        error!(error = ?e, "Failed to persist instance assignment");
        return error_response(500, "Internal server error");
    }
    
    success_response(None)
}

fn parse_instance_id(req: &Request) -> Result<InstanceId> {
    let id_str = match req.path_parameters().first("id") {
        Some(id) => id.to_string(),
        None => {
            warn!(path = %req.uri().path(), "Missing path parameter: `id`");
            bail!("Missing path parameter: id")
        }
    };

    match InstanceId::new(id_str.clone()) {
        Ok(id) => Ok(id),
        Err(e) => {
            warn!(invalid_id = %id_str, "Failed to parse instance ID");
            bail!("Invalid instance ID: {}", e)
        }
    }
}
