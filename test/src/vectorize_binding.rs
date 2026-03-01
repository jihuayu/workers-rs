use crate::SomeSharedData;
use serde_json::json;
use worker::{Env, Request, Response, Result};

pub async fn vectorize_binding_ok(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let vectorize = env.vectorize("VECTORIZE")?;
    let result: serde_json::Value = vectorize
        .query(json!({ "vector": [1.0, 2.0, 3.0], "topK": 1 }))
        .await?;
    Response::from_json(&result)
}

pub async fn vectorize_binding_missing(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let err = env
        .vectorize("MISSING_VECTORIZE")
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "expected missing binding error".to_string());
    Response::ok(err)
}
