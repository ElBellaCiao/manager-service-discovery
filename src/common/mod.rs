use lambda_http::{Body, Response};
use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;
use tracing::error;

pub fn success_response(result: impl Serialize) -> Response<Body> {
    let body = match serde_json::to_string(&result) {
        Ok(body) => body,
        Err(e) => {
            error!(error = ?e, "Failed to serialize response body");
            return error_response(500, "Internal server error");
        }
    };

    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(body.into())
        .unwrap()
}

pub fn error_response(status_code: u16, error: impl Debug) -> Response<Body> {
    let payload = json!({ "error": format!("{:?}", error) });
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload).unwrap().into())
        .unwrap()
}
