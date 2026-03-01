use crate::send::SendFuture;
use crate::{env::EnvBinding, Error, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker_sys::WorkflowBinding as WorkflowBindingSys;

#[derive(Debug, Clone)]
pub struct WorkflowBinding(WorkflowBindingSys);

unsafe impl Send for WorkflowBinding {}
unsafe impl Sync for WorkflowBinding {}

impl WorkflowBinding {
    pub async fn trigger<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        let promise = self.0.trigger(serde_wasm_bindgen::to_value(&input)?)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }

    pub async fn get_status<U: DeserializeOwned>(&self, workflow_id: &str) -> Result<U> {
        let promise = self.0.get_status(workflow_id)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

impl EnvBinding for WorkflowBinding {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to WorkflowBinding from non-object value".into());
        }

        let has_trigger = js_sys::Reflect::has(&val, &JsValue::from("trigger"))?;
        if !has_trigger {
            return Err("Binding cannot be cast to WorkflowBinding: missing `trigger` method".into());
        }

        Ok(Self(val.unchecked_into()))
    }
}

impl JsCast for WorkflowBinding {
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

impl AsRef<JsValue> for WorkflowBinding {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl From<JsValue> for WorkflowBinding {
    fn from(val: JsValue) -> Self {
        Self(val.unchecked_into())
    }
}

impl From<WorkflowBinding> for JsValue {
    fn from(value: WorkflowBinding) -> Self {
        value.0.into()
    }
}
