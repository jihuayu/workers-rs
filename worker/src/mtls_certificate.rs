use crate::{env::EnvBinding, Result};
use wasm_bindgen::{JsCast, JsValue};
use worker_sys::MtlsCertificate as MtlsCertificateSys;

#[derive(Debug, Clone)]
pub struct MtlsCertificate(MtlsCertificateSys);

unsafe impl Send for MtlsCertificate {}
unsafe impl Sync for MtlsCertificate {}

impl MtlsCertificate {
    pub fn id(&self) -> Option<String> {
        js_sys::Reflect::get(self.as_ref(), &JsValue::from("id"))
            .ok()
            .and_then(|v| v.as_string())
    }

    pub fn as_request_cf_value(&self) -> JsValue {
        self.as_ref().clone()
    }
}

impl EnvBinding for MtlsCertificate {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to MtlsCertificate from non-object value".into());
        }

        Ok(Self(val.unchecked_into()))
    }
}

impl JsCast for MtlsCertificate {
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

impl AsRef<JsValue> for MtlsCertificate {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl From<JsValue> for MtlsCertificate {
    fn from(val: JsValue) -> Self {
        Self(val.unchecked_into())
    }
}

impl From<MtlsCertificate> for JsValue {
    fn from(value: MtlsCertificate) -> Self {
        value.0.into()
    }
}
