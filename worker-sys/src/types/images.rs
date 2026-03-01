use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ImagesBinding;

    #[wasm_bindgen(method, catch)]
    pub fn transform(
        this: &ImagesBinding,
        input: JsValue,
    ) -> Result<js_sys::Promise, JsValue>;
}
