use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use wasm_bindgen::{JsCast, JsValue};
use worker_sys::DynamicDispatcher as DynamicDispatcherSys;

use crate::{env::EnvBinding, Fetcher, Result};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DispatchGetOptions {
    #[serde(flatten)]
    pub extra: HashMap<String, JsonValue>,
}

/// A binding for dispatching events to Workers inside of a dispatch namespace by their name. This
/// allows for your worker to directly invoke many workers by name instead of having multiple
/// service worker bindings.
///
/// # Example:
///
/// ```no_run
/// let dispatcher = env.dynamic_dispatcher("DISPATCHER")?;
/// let fetcher = dispatcher.get("namespaced-worker-name")?;
/// let resp = fetcher.fetch_request(req).await?;
/// ```
#[derive(Debug, Clone)]
pub struct DynamicDispatcher(DynamicDispatcherSys);

impl DynamicDispatcher {
    /// Gets a [Fetcher] for a Worker inside of the dispatch namespace based of the name specified.
    pub fn get(&self, name: impl Into<String>) -> Result<Fetcher> {
        let fetcher_sys = self
            .0
            .get(name.into(), JsValue::undefined(), JsValue::undefined())?;
        Ok(fetcher_sys.into())
    }

    /// Gets a [Fetcher] and passes `args` to the dispatch namespace `get(...)` call.
    pub fn get_with_args<T: Serialize>(&self, name: impl Into<String>, args: T) -> Result<Fetcher> {
        let args_js = to_js_value(&args)?;
        let fetcher_sys = self.0.get(name.into(), args_js, JsValue::undefined())?;
        Ok(fetcher_sys.into())
    }

    /// Gets a [Fetcher] and passes both `args` and `options` to the dispatch namespace `get(...)` call.
    pub fn get_with_options<T: Serialize>(
        &self,
        name: impl Into<String>,
        args: T,
        options: DispatchGetOptions,
    ) -> Result<Fetcher> {
        let args_js = to_js_value(&args)?;
        let options_js = match dispatch_options_json(options) {
            Some(options) => to_js_value(&options)?,
            None => JsValue::undefined(),
        };
        let fetcher_sys = self.0.get(name.into(), args_js, options_js)?;
        Ok(fetcher_sys.into())
    }
}

fn to_js_value<T: Serialize>(value: &T) -> Result<JsValue> {
    value
        .serialize(&serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true))
        .map_err(Into::into)
}

fn dispatch_options_json(options: DispatchGetOptions) -> Option<JsonValue> {
    if options.extra.is_empty() {
        None
    } else {
        Some(JsonValue::Object(options.extra.into_iter().collect()))
    }
}

impl EnvBinding for DynamicDispatcher {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to DynamicDispatcher from non-object value".into());
        }

        let has_get = js_sys::Reflect::has(&val, &JsValue::from("get"))?;
        if !has_get {
            return Err("Binding cannot be cast to DynamicDispatcher: missing `get` method".into());
        }

        Ok(Self(val.unchecked_into()))
    }
}

impl JsCast for DynamicDispatcher {
    fn instanceof(val: &JsValue) -> bool {
        val.is_object()
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        DynamicDispatcher(val.unchecked_into())
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const Self) }
    }
}

impl AsRef<JsValue> for DynamicDispatcher {
    fn as_ref(&self) -> &wasm_bindgen::JsValue {
        &self.0
    }
}

impl From<JsValue> for DynamicDispatcher {
    fn from(val: JsValue) -> Self {
        DynamicDispatcher(val.unchecked_into())
    }
}

impl From<DynamicDispatcher> for JsValue {
    fn from(sec: DynamicDispatcher) -> Self {
        sec.0.into()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{dispatch_options_json, DispatchGetOptions};

    #[test]
    fn empty_options_serialize_to_none() {
        assert!(dispatch_options_json(DispatchGetOptions::default()).is_none());
    }

    #[test]
    fn dispatch_options_preserve_extra_fields() {
        let mut extra = HashMap::new();
        extra.insert("trace".to_string(), serde_json::json!(true));

        let options = dispatch_options_json(DispatchGetOptions { extra })
            .expect("options should be present");

        assert_eq!(options["trace"], serde_json::json!(true));
    }
}
