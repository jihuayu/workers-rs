use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;

use crate::{Error, Result};

pub fn to_js_value<T: Serialize>(value: &T) -> Result<JsValue> {
    serde_wasm_bindgen::to_value(value)
        .map_err(|err| Error::RustError(format!("RPC argument serialization failed: {err}")))
}

pub fn from_js_value<T: DeserializeOwned>(value: JsValue) -> Result<T> {
    serde_wasm_bindgen::from_value(value)
        .map_err(|err| Error::RustError(format!("RPC return deserialization failed: {err}")))
}
