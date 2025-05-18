use crate::common::response::{error_response, success_response};
use crate::model::discoverability_request::{GetAssignmentRequest, PutAssignmentRequest, PutAssignmentRequestBody};
use crate::model::InstanceId;
use crate::service::discoverability_service;
use crate::service::discoverability_service::Deps;
use lambda_http::{Body, Request, RequestExt, Response};

pub async fn handle_get(req: Request, deps: Deps) -> Response<Body> {
    let instance_id = match parse_instance_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let get_assignment_request = GetAssignmentRequest { instance_id };

    let result = match discoverability_service::get_assignment(get_assignment_request, deps).await {
        Ok(val) => val,
        Err(e) => return error_response(500, format!("Internal error: {}", e)),
    };

    match serde_json::to_string(&result) {
        Ok(body) => success_response(Some(body.into())),
        Err(e) => error_response(500, format!("Serialization error: {}", e)),
    }
}

pub async fn handle_put(req: Request, deps: Deps) -> Response<Body> {
    let instance_id = match parse_instance_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let body: PutAssignmentRequestBody = match serde_json::from_slice(req.body().as_ref()) {
        Ok(b) => b,
        Err(e) => return error_response(400, format!("Invalid JSON body: {}", e)),
    };

    let put_request = PutAssignmentRequest {
        instance_id,
        stocks: body.stocks,
        input: body.input,
        output: body.output,
        expire_at: body.expire_at,
    };

    if let Err(e) = discoverability_service::put_assignment(put_request, deps).await {
        return error_response(500, format!("Internal error: {}", e));
    }
    
    success_response(None)
}

fn parse_instance_id(req: &Request) -> Result<InstanceId, Response<Body>> {
    let id_str = req.path_parameters().first("id")
        .ok_or_else(|| error_response(400, "Missing path parameter: id"))?
        .to_string();

    InstanceId::new(id_str)
        .map_err(|e| error_response(400, format!("Invalid instance ID: {}", e)))
}
