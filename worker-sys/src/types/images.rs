use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ImagesBinding;

    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ImageTransformer;

    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ImageTransformationResult;

    #[wasm_bindgen(method, catch)]
    pub fn info(
        this: &ImagesBinding,
        input: JsValue,
        options: JsValue,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn input(
        this: &ImagesBinding,
        input: JsValue,
        options: JsValue,
    ) -> Result<ImageTransformer, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn transform(
        this: &ImagesBinding,
        input: JsValue,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name = transform)]
    pub fn transform_step(
        this: &ImageTransformer,
        options: JsValue,
    ) -> Result<ImageTransformer, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn draw(this: &ImageTransformer, image: JsValue) -> Result<ImageTransformer, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn output(
        this: &ImageTransformer,
        options: JsValue,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, getter, js_name = response)]
    pub fn response(this: &ImageTransformationResult) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = contentType)]
    pub fn content_type(this: &ImageTransformationResult) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = image)]
    pub fn image(this: &ImageTransformationResult) -> JsValue;
}
