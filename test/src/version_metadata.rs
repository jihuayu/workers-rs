use crate::SomeSharedData;
use serde_json::json;
use worker::{Env, Request, Response, Result};

pub async fn version_metadata_ok(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let metadata = env.version_metadata("VERSION_METADATA")?;
    Response::from_json(&json!({
        "id": metadata.id(),
        "tag": metadata.tag(),
        "timestamp": metadata.timestamp(),
    }))
}

pub async fn version_metadata_missing(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let err = env
        .version_metadata("MISSING_VERSION_METADATA")
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "expected missing binding error".to_string());
    Response::ok(err)
}
