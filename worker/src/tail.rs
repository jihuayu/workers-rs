use wasm_bindgen::{JsCast, JsValue};
use worker_sys::TailEvent as TailEventSys;

#[derive(Debug, Clone)]
pub struct TailEvent(TailEventSys);

unsafe impl Send for TailEvent {}
unsafe impl Sync for TailEvent {}

impl From<TailEventSys> for TailEvent {
    fn from(value: TailEventSys) -> Self {
        Self(value)
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
