use crate::send::SendFuture;
use crate::{env::EnvBinding, Error, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker_sys::{
    ImageTransformationResult as ImageTransformationResultSys, ImageTransformer as ImageTransformerSys,
    ImagesBinding as ImagesBindingSys,
};

#[derive(Debug, Clone)]
pub struct ImagesBinding(ImagesBindingSys);

#[derive(Debug, Clone)]
pub struct ImageTransformer(ImageTransformerSys);

#[derive(Debug, Clone)]
pub struct ImageTransformationResult(ImageTransformationResultSys);

unsafe impl Send for ImagesBinding {}
unsafe impl Sync for ImagesBinding {}
unsafe impl Send for ImageTransformer {}
unsafe impl Sync for ImageTransformer {}
unsafe impl Send for ImageTransformationResult {}
unsafe impl Sync for ImageTransformationResult {}

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

    pub async fn info(&self, input: JsValue, options: Option<JsValue>) -> Result<JsValue> {
        let promise = self
            .0
            .info(input, options.unwrap_or(JsValue::UNDEFINED))?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        output.map_err(Error::from)
    }

    pub async fn info_json<T: Serialize, U: DeserializeOwned>(
        &self,
        input: T,
        options: Option<serde_json::Value>,
    ) -> Result<U> {
        let input = serde_wasm_bindgen::to_value(&input)?;
        let options = options
            .map(|opt| serde_wasm_bindgen::to_value(&opt))
            .transpose()?;
        let value = self.info(input, options).await?;
        Ok(serde_wasm_bindgen::from_value(value)?)
    }

    pub fn input(&self, input: JsValue, options: Option<JsValue>) -> Result<ImageTransformer> {
        let transformer = self
            .0
            .input(input, options.unwrap_or(JsValue::UNDEFINED))?;
        Ok(ImageTransformer(transformer))
    }

    pub fn input_json<T: Serialize>(
        &self,
        input: T,
        options: Option<serde_json::Value>,
    ) -> Result<ImageTransformer> {
        let input = serde_wasm_bindgen::to_value(&input)?;
        let options = options
            .map(|opt| serde_wasm_bindgen::to_value(&opt))
            .transpose()?;
        self.input(input, options)
    }
}

impl ImageTransformer {
    pub fn transform(self, options: JsValue) -> Result<Self> {
        Ok(Self(self.0.transform_step(options)?))
    }

    pub fn transform_json(self, options: serde_json::Value) -> Result<Self> {
        self.transform(serde_wasm_bindgen::to_value(&options)?)
    }

    pub fn draw(self, image: JsValue) -> Result<Self> {
        Ok(Self(self.0.draw(image)?))
    }

    pub async fn output(self, options: Option<JsValue>) -> Result<ImageTransformationResult> {
        let promise = self.0.output(options.unwrap_or(JsValue::UNDEFINED))?;
        let output = SendFuture::new(JsFuture::from(promise)).await;
        let value = output.map_err(Error::from)?;
        Ok(ImageTransformationResult(value.unchecked_into()))
    }

    pub async fn output_json(self, options: Option<serde_json::Value>) -> Result<ImageTransformationResult> {
        let options = options
            .map(|opt| serde_wasm_bindgen::to_value(&opt))
            .transpose()?;
        self.output(options).await
    }
}

impl ImageTransformationResult {
    pub fn response_raw(&self) -> JsValue {
        self.0.response()
    }

    pub fn content_type(&self) -> Option<String> {
        self.0.content_type()
    }

    pub fn image_raw(&self) -> JsValue {
        self.0.image()
    }

    pub fn image_json<U: DeserializeOwned>(&self) -> Result<U> {
        Ok(serde_wasm_bindgen::from_value(self.0.image())?)
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
