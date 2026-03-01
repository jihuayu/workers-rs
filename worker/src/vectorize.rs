use crate::send::SendFuture;
use crate::{env::EnvBinding, Error, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker_sys::VectorizeIndex as VectorizeIndexSys;

#[derive(Debug, Clone)]
pub struct VectorizeIndex(VectorizeIndexSys);

unsafe impl Send for VectorizeIndex {}
unsafe impl Sync for VectorizeIndex {}

impl VectorizeIndex {
    pub async fn upsert<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        let promise = self.0.upsert(serde_wasm_bindgen::to_value(&input)?)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }

    pub async fn query<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        let promise = self.0.query(serde_wasm_bindgen::to_value(&input)?)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }

    pub async fn delete<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        let promise = self.0.delete(serde_wasm_bindgen::to_value(&input)?)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

impl EnvBinding for VectorizeIndex {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to VectorizeIndex from non-object value".into());
        }

        let has_query = js_sys::Reflect::has(&val, &JsValue::from("query"))?;
        if !has_query {
            return Err("Binding cannot be cast to VectorizeIndex: missing `query` method".into());
        }

        Ok(Self(val.unchecked_into()))
    }
}

impl JsCast for VectorizeIndex {
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

impl AsRef<JsValue> for VectorizeIndex {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl From<JsValue> for VectorizeIndex {
    fn from(val: JsValue) -> Self {
        Self(val.unchecked_into())
    }
}

impl From<VectorizeIndex> for JsValue {
    fn from(value: VectorizeIndex) -> Self {
        value.0.into()
    }
}
