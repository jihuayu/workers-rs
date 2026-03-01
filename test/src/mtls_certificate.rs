use crate::SomeSharedData;
use worker::{js_sys, wasm_bindgen, web_sys, Env, Request, RequestInit, Response, Result};

pub async fn mtls_certificate_ok(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let cert = env.mtls_certificate("MTLS_CERTIFICATE")?;
    Response::ok(cert.id().unwrap_or_else(|| "missing-id".to_string()))
}

pub async fn mtls_certificate_missing(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let err = env
        .mtls_certificate("MISSING_MTLS_CERTIFICATE")
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "expected missing binding error".to_string());
    Response::ok(err)
}

pub async fn mtls_certificate_request_init(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let cert = env.mtls_certificate("MTLS_CERTIFICATE")?;
    let mut init = RequestInit::new();
    init.with_mtls_certificate(&cert);

    let init: web_sys::RequestInit = (&init).into();
    let cf = js_sys::Reflect::get(init.as_ref(), &wasm_bindgen::JsValue::from("cf"))?;
    let mtls = js_sys::Reflect::get(&cf, &wasm_bindgen::JsValue::from("mtlsCertificate"))?;
    let has_value = !mtls.is_undefined() && !mtls.is_null();
    Response::ok(has_value.to_string())
}
