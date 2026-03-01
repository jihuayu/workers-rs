use crate::SomeSharedData;
use worker::{ConnectionBuilder, Env, Request, Response, Result, SecureTransport, SocketOptions};

pub async fn hyperdrive_connect_ok(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let hyperdrive = env.hyperdrive("HYPERDRIVE")?;

    let socket = hyperdrive.connect_with_options(SocketOptions {
        secure_transport: SecureTransport::On,
        allow_half_open: false,
    })?;

    let _ = socket;
    let _ = ConnectionBuilder::new();
    Response::ok("connected")
}

pub async fn hyperdrive_connect_missing(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let err = env
        .hyperdrive("MISSING_HYPERDRIVE")
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "expected missing binding error".to_string());
    Response::ok(err)
}
