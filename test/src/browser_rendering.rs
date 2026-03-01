use crate::SomeSharedData;
use serde_json::json;
use worker::{Env, Request, Response, Result};

pub async fn browser_rendering_ok(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let browser = env.browser_rendering("BROWSER_RENDERING")?;
    let output: serde_json::Value = browser
        .render_json(json!({"url": "https://example.com"}))
        .await?;
    Response::from_json(&output)
}

pub async fn browser_rendering_missing(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let err = env
        .browser_rendering("MISSING_BROWSER_RENDERING")
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "expected missing binding error".to_string());
    Response::ok(err)
}
