use crate::{env::EnvBinding, send::SendFuture, Error, Result};
use serde::de::DeserializeOwned;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker_sys::{EmailMessage as EmailMessageSys, SendEmail as SendEmailSys};

#[derive(Debug, Clone)]
pub struct EmailMessage(EmailMessageSys);

#[derive(Debug, Clone)]
pub struct SendEmail(SendEmailSys);

unsafe impl Send for EmailMessage {}
unsafe impl Sync for EmailMessage {}
unsafe impl Send for SendEmail {}
unsafe impl Sync for SendEmail {}

impl EmailMessage {
    pub fn from_address(&self) -> Option<String> {
        self.0.from()
    }

    pub fn to_address(&self) -> Option<String> {
        self.0.to()
    }

    pub fn raw(&self) -> JsValue {
        self.0.raw()
    }

    pub fn headers(&self) -> JsValue {
        self.0.headers()
    }

    pub fn raw_size(&self) -> Option<u64> {
        self.0.raw_size().map(|v| v.round() as u64)
    }

    pub fn set_reject(&self, reason: &str) -> Result<()> {
        self.0.set_reject(reason).map_err(Error::from)
    }

    pub async fn forward<U: DeserializeOwned>(
        &self,
        rcpt_to: &str,
        headers: Option<JsValue>,
    ) -> Result<U> {
        let promise = self
            .0
            .forward(rcpt_to, headers.unwrap_or(JsValue::UNDEFINED))?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }

    pub async fn reply<U: DeserializeOwned>(&self, message: &EmailMessage) -> Result<U> {
        let promise = self.0.reply(&message.0)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

impl SendEmail {
    pub async fn send<U: DeserializeOwned>(&self, message: &EmailMessage) -> Result<U> {
        let promise = self.0.send(&message.0)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

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

impl EnvBinding for EmailMessage {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to EmailMessage from non-object value".into());
        }

        let has_from = js_sys::Reflect::has(&val, &JsValue::from("from"))?;
        let has_to = js_sys::Reflect::has(&val, &JsValue::from("to"))?;
        if !has_from || !has_to {
            return Err("Binding cannot be cast to EmailMessage: missing `from` or `to` field".into());
        }

        Ok(Self(val.unchecked_into()))
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

impl JsCast for SendEmail {
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

impl AsRef<JsValue> for SendEmail {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl EnvBinding for SendEmail {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to SendEmail from non-object value".into());
        }

        let has_send = js_sys::Reflect::has(&val, &JsValue::from("send"))?;
        if !has_send {
            return Err("Binding cannot be cast to SendEmail: missing `send` method".into());
        }

        Ok(Self(val.unchecked_into()))
    }
}

impl From<JsValue> for SendEmail {
    fn from(val: JsValue) -> Self {
        Self(val.unchecked_into())
    }
}

impl From<SendEmail> for JsValue {
    fn from(value: SendEmail) -> Self {
        value.0.into()
    }
}
