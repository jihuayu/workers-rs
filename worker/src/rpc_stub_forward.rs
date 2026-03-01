use std::collections::HashMap;

use wasm_bindgen::{JsCast, JsValue};

use crate::{Fetcher, Result, Stub};

#[derive(Debug, Clone)]
pub struct RpcStubForward {
    target: JsValue,
    namespace: Option<String>,
    metadata: HashMap<String, String>,
}

impl RpcStubForward {
    pub fn new(target: JsValue) -> Self {
        Self {
            target,
            namespace: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn into_rpc<T: JsCast>(self) -> T {
        self.target.unchecked_into()
    }

    pub fn to_wire(&self) -> Result<JsValue> {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from("version"), &JsValue::from("1"))?;
        js_sys::Reflect::set(&obj, &JsValue::from("target"), &self.target)?;

        if let Some(namespace) = &self.namespace {
            js_sys::Reflect::set(&obj, &JsValue::from("namespace"), &JsValue::from(namespace))?;
        }

        let metadata = serde_wasm_bindgen::to_value(&self.metadata)?;
        js_sys::Reflect::set(&obj, &JsValue::from("metadata"), &metadata)?;

        Ok(obj.into())
    }

    pub fn from_wire(wire: JsValue) -> Result<Self> {
        let target = js_sys::Reflect::get(&wire, &JsValue::from("target"))?;

        let namespace = js_sys::Reflect::get(&wire, &JsValue::from("namespace"))?
            .as_string();

        let metadata_value = js_sys::Reflect::get(&wire, &JsValue::from("metadata"))?;
        let metadata = if metadata_value.is_undefined() || metadata_value.is_null() {
            HashMap::new()
        } else {
            serde_wasm_bindgen::from_value(metadata_value)?
        };

        Ok(Self {
            target,
            namespace,
            metadata,
        })
    }
}

impl From<Fetcher> for RpcStubForward {
    fn from(value: Fetcher) -> Self {
        let target: JsValue = value.into_rpc();
        Self::new(target)
    }
}

impl From<Stub> for RpcStubForward {
    fn from(value: Stub) -> Self {
        let target: JsValue = value.into_rpc();
        Self::new(target)
    }
}
