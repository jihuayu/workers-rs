use crate::SomeSharedData;
use std::collections::HashMap;
use worker::{DispatchGetOptions, Env, Request, Response, Result};

pub async fn dynamic_dispatch_with_args(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let dispatcher = env.dynamic_dispatcher("DISPATCHER")?;
    let fetcher = dispatcher.get_with_args("svc-1", serde_json::json!({ "tenant": "acme" }))?;

    let req = Request::new("https://example.com/", worker::Method::Get)?;
    let mut resp = fetcher.fetch_request(req).await?;
    let body = resp.json::<serde_json::Value>().await?;

    Response::from_json(&body)
}

pub async fn dynamic_dispatch_with_options(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let dispatcher = env.dynamic_dispatcher("DISPATCHER")?;
    let mut extra = HashMap::new();
    extra.insert("trace".to_string(), serde_json::json!(true));
    extra.insert("region".to_string(), serde_json::json!("eu"));

    let fetcher = dispatcher.get_with_options(
        "svc-2",
        serde_json::json!(["alpha", "beta"]),
        DispatchGetOptions { extra },
    )?;

    let req = Request::new("https://example.com/", worker::Method::Get)?;
    let mut resp = fetcher.fetch_request(req).await?;
    let body = resp.json::<serde_json::Value>().await?;

    Response::from_json(&body)
}
