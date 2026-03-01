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

pub async fn images_binding_info(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let images = env.images("IMAGES")?;
    let output: serde_json::Value = images
        .info_json(json!({ "source": "raw-image" }), None)
        .await?;
    Response::from_json(&output)
}

pub async fn images_binding_pipeline(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let images = env.images("IMAGES")?;

    let result = images
        .input_json(json!({ "source": "raw-image" }), None)?
        .transform_json(json!({ "resize": { "width": 100, "height": 100 } }))?
        .draw(serde_wasm_bindgen::to_value(&json!({ "overlay": "logo" }))?)?
        .output_json(None)
        .await?;

    let image: serde_json::Value = result.image_json()?;

    Response::from_json(&json!({
        "contentType": result.content_type(),
        "image": image,
    }))
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
