use crate::SomeSharedData;
use serde_json::json;
use worker::{Env, Request, Response, Result};

pub async fn images_binding_ok(_req: Request, env: Env, _data: SomeSharedData) -> Result<Response> {
    let images = env.images("IMAGES")?;
    let output: serde_json::Value = images
        .transform_json(json!({ "source": "raw-image" }))
        .await?;
    Response::from_json(&output)
}

pub async fn images_binding_missing(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let err = env
        .images("MISSING_IMAGES")
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "expected missing binding error".to_string());
    Response::ok(err)
}
