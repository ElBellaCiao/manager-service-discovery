use cloud_util::Ec2;
use aws_lambda_events::apigw::{ApiGatewayV2httpRequest, ApiGatewayV2httpResponse};
use lambda_runtime::{run, service_fn, LambdaEvent, Error};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    tags: std::collections::HashMap<String, String>,
}

#[derive(Serialize)]
struct Output {
    instances: Vec<String>,
}

async fn get_ec2_instances(input: Input) -> Result<Vec<String>> {
    let ec2_client = Ec2::new(None).await;
    ec2_client.get_instances_by_tags(&input.tags).await
}

async fn process(payload: ApiGatewayV2httpRequest) -> Result<ApiGatewayV2httpResponse, Error> {
    let raw = payload.body.ok_or_else(|| anyhow!("missing body"))?;
    let input = serde_json::from_str::<Input>(&raw)?;

    let instances = get_ec2_instances(input).await?;

    let output = Output { instances };
    let body = serde_json::to_string(&output)
        .context("failed to serialize response JSON")?;

    Ok(ApiGatewayV2httpResponse {
        status_code: 200,
        body: Some(body.into()),
        ..Default::default()
    })
}

async fn function_handler(event: LambdaEvent<ApiGatewayV2httpRequest>) -> Result<ApiGatewayV2httpResponse, Error> {
    println!("{:?}", event);

    match process(event.payload).await {
        Ok(resp) => Ok(resp),
        Err(err) => {
            eprintln!("Error processing request: {:?}", err);

            Ok(ApiGatewayV2httpResponse {
                status_code: 500,
                body: None,
                ..Default::default()
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}