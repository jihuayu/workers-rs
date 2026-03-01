use crate::SomeSharedData;
use worker::{Env, Request, Response, Result};

pub async fn dynamic_worker_loader_ok(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let loader = env.dynamic_worker_loader("DYNAMIC_WORKER_LOADER")?;
    let output: serde_json::Value = loader.load_json("sample-worker").await?;
    Response::from_json(&output)
}

pub async fn dynamic_worker_loader_missing(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let err = env
        .dynamic_worker_loader("MISSING_DYNAMIC_WORKER_LOADER")
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "expected missing binding error".to_string());
    Response::ok(err)
}
