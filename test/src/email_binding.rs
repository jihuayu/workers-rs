use crate::SomeSharedData;
use serde_json::json;
use worker::{Env, EmailMessage, Request, Response, Result};

pub async fn email_binding_message_ok(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let message = env.get_binding::<EmailMessage>("EMAIL_MESSAGE")?;
    message.set_reject("blocked by policy")?;

    let forwarded: serde_json::Value = message.forward("forward@example.com", None).await?;
    let replied: serde_json::Value = message.reply(&message).await?;

    Response::from_json(&json!({
        "from": message.from_address(),
        "to": message.to_address(),
        "rawSize": message.raw_size(),
        "forward": forwarded,
        "reply": replied,
    }))
}

pub async fn email_binding_send_ok(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let sender = env.send_email("EMAIL_SENDER")?;
    let message = env.get_binding::<EmailMessage>("EMAIL_MESSAGE")?;

    let sent: serde_json::Value = sender.send(&message).await?;
    Response::from_json(&sent)
}

pub async fn email_binding_send_missing(
    _req: Request,
    env: Env,
    _data: SomeSharedData,
) -> Result<Response> {
    let err = env
        .send_email("MISSING_EMAIL_SENDER")
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "expected missing email sender binding error".to_string());
    Response::ok(err)
}
