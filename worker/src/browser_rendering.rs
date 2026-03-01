use crate::send::SendFuture;
use crate::{env::EnvBinding, Error, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker_sys::BrowserRendering as BrowserRenderingSys;

#[derive(Debug, Clone)]
pub struct BrowserRendering(BrowserRenderingSys);

unsafe impl Send for BrowserRendering {}
unsafe impl Sync for BrowserRendering {}

impl BrowserRendering {
    pub async fn render(&self, input: JsValue) -> Result<JsValue> {
        let promise = self.0.render(input)?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        output.map_err(Error::from)
    }

    pub async fn render_json<T: Serialize, U: DeserializeOwned>(&self, input: T) -> Result<U> {
        let output = self.render(serde_wasm_bindgen::to_value(&input)?).await?;
        Ok(serde_wasm_bindgen::from_value(output)?)
    }
}

impl EnvBinding for BrowserRendering {
    const TYPE_NAME: &'static str = "Object";

    fn get(val: JsValue) -> Result<Self> {
        if !val.is_object() {
            return Err("Binding cannot be cast to BrowserRendering from non-object value".into());
        }

        let has_render = js_sys::Reflect::has(&val, &JsValue::from("render"))?;
        if !has_render {
            return Err("Binding cannot be cast to BrowserRendering: missing `render` method".into());
        }

        Ok(Self(val.unchecked_into()))
    }
}

impl JsCast for BrowserRendering {
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

impl AsRef<JsValue> for BrowserRendering {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl From<JsValue> for BrowserRendering {
    fn from(val: JsValue) -> Self {
        Self(val.unchecked_into())
    }
}

impl From<BrowserRendering> for JsValue {
    fn from(value: BrowserRendering) -> Self {
        value.0.into()
    }
}
