use crate::send::SendFuture;
use crate::{env::EnvBinding, Error, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker_sys::VectorizeIndex as VectorizeIndexSys;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct VectorizeQueryOptions {
    #[serde(rename = "topK", skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "returnValues", skip_serializing_if = "Option::is_none")]
    pub return_values: Option<bool>,
    #[serde(rename = "returnMetadata", skip_serializing_if = "Option::is_none")]
    pub return_metadata: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VectorizeGetByIdsRequest {
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VectorizeDeleteByIdsRequest {
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VectorizeIndex(VectorizeIndexSys);

unsafe impl Send for VectorizeIndex {}
unsafe impl Sync for VectorizeIndex {}

impl VectorizeIndex {
    async fn invoke_without_input<U: DeserializeOwned>(
        &self,
        method: impl FnOnce(&VectorizeIndexSys) -> std::result::Result<js_sys::Promise, JsValue>,
    ) -> Result<U> {
        let promise = method(&self.0)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }

    async fn invoke_with_input<T: Serialize, U: DeserializeOwned>(
        &self,
        input: T,
        method: impl FnOnce(
            &VectorizeIndexSys,
            JsValue,
        ) -> std::result::Result<js_sys::Promise, JsValue>,
    ) -> Result<U> {
        let promise = method(&self.0, serde_wasm_bindgen::to_value(&input)?)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }

    pub async fn upsert<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        self.invoke_with_input(input, |inner, value| inner.upsert(value))
            .await
    }

    pub async fn describe<U: DeserializeOwned>(&self) -> Result<U> {
        self.invoke_without_input(|inner| inner.describe()).await
    }

    pub async fn query<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        self.invoke_with_input(input, |inner, value| inner.query(value))
            .await
    }

    pub async fn get_by_ids<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        self.invoke_with_input(input, |inner, value| inner.get_by_ids(value))
            .await
    }

    pub async fn delete<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        self.invoke_with_input(input, |inner, value| inner.delete(value))
            .await
    }

    pub async fn delete_by_ids<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        self.invoke_with_input(input, |inner, value| inner.delete_by_ids(value))
            .await
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

#[cfg(test)]
mod tests {
    use super::{VectorizeDeleteByIdsRequest, VectorizeGetByIdsRequest, VectorizeQueryOptions};

    #[test]
    fn query_options_uses_top_k_field_name() {
        let options = VectorizeQueryOptions {
            top_k: Some(8),
            namespace: Some("ns-1".to_string()),
            return_values: Some(true),
            return_metadata: None,
        };

        let serialized = serde_json::to_value(options).expect("query options should serialize");
        assert_eq!(serialized.get("topK").and_then(|v| v.as_u64()), Some(8));
        assert_eq!(
            serialized.get("namespace").and_then(|v| v.as_str()),
            Some("ns-1")
        );
        assert_eq!(
            serialized.get("returnValues").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert!(serialized.get("returnMetadata").is_none());
    }

    #[test]
    fn id_request_types_serialize_ids_and_namespace() {
        let get_by_ids = VectorizeGetByIdsRequest {
            ids: vec!["id-1".to_string(), "id-2".to_string()],
            namespace: Some("ns-2".to_string()),
        };
        let delete_by_ids = VectorizeDeleteByIdsRequest {
            ids: vec!["id-3".to_string()],
            namespace: None,
        };

        let get_serialized = serde_json::to_value(get_by_ids).expect("request should serialize");
        let delete_serialized =
            serde_json::to_value(delete_by_ids).expect("request should serialize");

        assert_eq!(get_serialized["ids"], serde_json::json!(["id-1", "id-2"]));
        assert_eq!(get_serialized["namespace"], serde_json::json!("ns-2"));
        assert_eq!(delete_serialized["ids"], serde_json::json!(["id-3"]));
        assert!(delete_serialized.get("namespace").is_none());
    }
}
