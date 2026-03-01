use crate::send::SendFuture;
use crate::{env::EnvBinding, Error, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker_sys::ImagesBinding as ImagesBindingSys;

#[derive(Debug, Clone)]
pub struct ImagesBinding(ImagesBindingSys);

unsafe impl Send for ImagesBinding {}
unsafe impl Sync for ImagesBinding {}

impl ImagesBinding {
    pub async fn transform(&self, input: JsValue) -> Result<JsValue> {
        let promise = self.0.transform(input)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        output.map_err(Error::from)
    }

    pub async fn transform_json<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        let value = self.transform(serde_wasm_bindgen::to_value(&input)?).await?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

impl EnvBinding for ImagesBinding {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to ImagesBinding from non-object value".into());
        }

        let has_transform = js_sys::Reflect::has(&val, &JsValue::from("transform"))?;
        if !has_transform {
            return Err("Binding cannot be cast to ImagesBinding: missing `transform` method".into());
        }

        Ok(Self(val.unchecked_into()))
    }
}

impl JsCast for ImagesBinding {
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

impl AsRef<JsValue> for ImagesBinding {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl From<JsValue> for ImagesBinding {
    fn from(val: JsValue) -> Self {
        Self(val.unchecked_into())
    }
}

impl From<ImagesBinding> for JsValue {
    fn from(value: ImagesBinding) -> Self {
        value.0.into()
    }
}
