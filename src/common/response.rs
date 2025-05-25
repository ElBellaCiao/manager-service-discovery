use std::fmt::Debug;
use serde_json::json;
use lambda_http::{Response, Body};

pub fn success_response(body: Option<Body>) -> Response<Body> {
    let body = body.unwrap_or_else(|| r#"{"message": "Success"}"#.into());

    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(body)
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