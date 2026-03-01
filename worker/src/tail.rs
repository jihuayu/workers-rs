use crate::{env::EnvBinding, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use wasm_bindgen::{JsCast, JsValue};
use worker_sys::TailEvent as TailEventSys;

#[derive(Debug, Clone)]
pub struct TailEvent(TailEventSys);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TailEventKind {
    Outcome,
    SpanOpen,
    SpanClose,
    DiagnosticChannel,
    Exception,
    Log,
    Return,
    Attributes,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedTailEvent {
    pub kind: TailEventKind,
    pub raw: JsonValue,
}

unsafe impl Send for TailEvent {}
unsafe impl Sync for TailEvent {}

impl TailEvent {
    pub fn invocation_id(&self) -> Option<String> {
        self.0.invocation_id()
    }

    pub fn span_context(&self) -> JsValue {
        self.0.span_context()
    }

    pub fn timestamp(&self) -> Option<f64> {
        self.0.timestamp()
    }

    pub fn sequence(&self) -> Option<u64> {
        sequence_from_js(self.0.sequence())
    }

    pub fn event_raw(&self) -> JsValue {
        self.0.event()
    }

    pub fn event_typed(&self) -> Result<TypedTailEvent> {
        let raw: JsonValue = serde_wasm_bindgen::from_value(self.0.event())?;
        Ok(parse_typed_tail_event(raw))
    }
}

fn sequence_from_js(sequence: Option<f64>) -> Option<u64> {
    sequence.map(|value| value.round() as u64)
}

fn parse_typed_tail_event(raw: JsonValue) -> TypedTailEvent {
    let kind = raw
        .get("type")
        .and_then(JsonValue::as_str)
        .map(parse_tail_event_kind)
        .unwrap_or_else(|| TailEventKind::Unknown("unknown".to_string()));
    TypedTailEvent { kind, raw }
}

fn parse_tail_event_kind(raw_type: &str) -> TailEventKind {
    match raw_type {
        "outcome" => TailEventKind::Outcome,
        "spanOpen" => TailEventKind::SpanOpen,
        "spanClose" => TailEventKind::SpanClose,
        "diagnosticChannel" => TailEventKind::DiagnosticChannel,
        "exception" => TailEventKind::Exception,
        "log" => TailEventKind::Log,
        "return" => TailEventKind::Return,
        "attributes" => TailEventKind::Attributes,
        other => TailEventKind::Unknown(other.to_string()),
    }
}

impl From<TailEventSys> for TailEvent {
    fn from(value: TailEventSys) -> Self {
        Self(value)
    }
}

impl EnvBinding for TailEvent {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to TailEvent from non-object value".into());
        }

        let has_event = js_sys::Reflect::has(&val, &JsValue::from("event"))?;
        if !has_event {
            return Err("Binding cannot be cast to TailEvent: missing `event` field".into());
        }

        Ok(Self(val.unchecked_into()))
    }
}

impl AsRef<JsValue> for TailEvent {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl JsCast for TailEvent {
    fn instanceof(val: &JsValue) -> bool {
        val.is_object()
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        Self(val.unchecked_into())
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const Self) }
    }
}

impl From<TailEvent> for JsValue {
    fn from(value: TailEvent) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_typed_tail_event, sequence_from_js, TailEventKind};

    #[test]
    fn known_tail_variant_is_parsed() {
        let parsed = parse_typed_tail_event(serde_json::json!({
            "type": "log",
            "message": "hello"
        }));
        assert_eq!(parsed.kind, TailEventKind::Log);
    }

    #[test]
    fn unknown_tail_variant_falls_back_safely() {
        let parsed = parse_typed_tail_event(serde_json::json!({
            "type": "newFutureType",
            "payload": { "x": 1 }
        }));
        assert_eq!(
            parsed.kind,
            TailEventKind::Unknown("newFutureType".to_string())
        );
    }

    #[test]
    fn sequence_conversion_rounds_from_js_number() {
        assert_eq!(sequence_from_js(Some(10.2)), Some(10));
        assert_eq!(sequence_from_js(Some(10.7)), Some(11));
        assert_eq!(sequence_from_js(None), None);
    }
}
