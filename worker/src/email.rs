use wasm_bindgen::{JsCast, JsValue};
use worker_sys::EmailMessage as EmailMessageSys;

#[derive(Debug, Clone)]
pub struct EmailMessage(EmailMessageSys);

unsafe impl Send for EmailMessage {}
unsafe impl Sync for EmailMessage {}

impl From<EmailMessageSys> for EmailMessage {
    fn from(value: EmailMessageSys) -> Self {
        Self(value)
    }
}

impl AsRef<JsValue> for EmailMessage {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl JsCast for EmailMessage {
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

impl From<EmailMessage> for JsValue {
    fn from(value: EmailMessage) -> Self {
        value.0.into()
    }
}
