use crate::SomeSharedData;
use serde_json::json;
use worker::{Env, Request, Response, Result};

pub async fn workflows_binding_ok(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let workflows = env.workflows("WORKFLOWS")?;
    let trigger: serde_json::Value = workflows
        .trigger(json!({ "name": "demo" }))
        .await?;
    Response::from_json(&trigger)
}

pub async fn workflows_binding_missing(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let err = env
        .workflows("MISSING_WORKFLOWS")
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "expected missing binding error".to_string());
    Response::ok(err)
}
