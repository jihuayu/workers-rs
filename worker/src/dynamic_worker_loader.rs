use crate::send::SendFuture;
use crate::{env::EnvBinding, Error, Result};
use serde::de::DeserializeOwned;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker_sys::DynamicWorkerLoader as DynamicWorkerLoaderSys;

#[derive(Debug, Clone)]
pub struct DynamicWorkerLoader(DynamicWorkerLoaderSys);

unsafe impl Send for DynamicWorkerLoader {}
unsafe impl Sync for DynamicWorkerLoader {}

impl DynamicWorkerLoader {
    pub async fn load(&self, worker_name: &str) -> Result<JsValue> {
        let promise = self.0.load(worker_name)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        output.map_err(Error::from)
    }

    pub async fn load_json<T: DeserializeOwned>(&self, worker_name: &str) -> Result<T> {
        let value = self.load(worker_name).await?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

impl EnvBinding for DynamicWorkerLoader {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to DynamicWorkerLoader from non-object value".into());
        }

        let has_load = js_sys::Reflect::has(&val, &JsValue::from("load"))?;
        if !has_load {
            return Err("Binding cannot be cast to DynamicWorkerLoader: missing `load` method".into());
        }

        Ok(Self(val.unchecked_into()))
    }
}

impl JsCast for DynamicWorkerLoader {
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

impl AsRef<JsValue> for DynamicWorkerLoader {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl From<JsValue> for DynamicWorkerLoader {
    fn from(val: JsValue) -> Self {
        Self(val.unchecked_into())
    }
}

impl From<DynamicWorkerLoader> for JsValue {
    fn from(value: DynamicWorkerLoader) -> Self {
        value.0.into()
    }
}
