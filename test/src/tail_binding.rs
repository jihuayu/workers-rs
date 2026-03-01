use crate::SomeSharedData;
use serde_json::json;
use worker::{Env, Request, Response, Result, TailEvent, TailEventKind};

pub async fn tail_binding_known(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let event = env.get_binding::<TailEvent>("TAIL_EVENT")?;
    let typed = event.event_typed()?;

    Response::from_json(&json!({
        "invocationId": event.invocation_id(),
        "timestamp": event.timestamp(),
        "sequence": event.sequence(),
        "kind": typed.kind,
        "raw": typed.raw,
    }))
}

pub async fn tail_binding_unknown(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let event = env.get_binding::<TailEvent>("TAIL_EVENT_UNKNOWN")?;
    let typed = event.event_typed()?;

    let kind = match typed.kind {
        TailEventKind::Unknown(kind) => kind,
        other => format!("unexpected:{other:?}"),
    };

    Response::from_json(&json!({
        "kind": kind,
        "raw": typed.raw,
    }))
}
