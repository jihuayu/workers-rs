use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TailEvent;

    #[wasm_bindgen(method, getter, js_name = invocationId)]
    pub fn invocation_id(this: &TailEvent) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = spanContext)]
    pub fn span_context(this: &TailEvent) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn timestamp(this: &TailEvent) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn sequence(this: &TailEvent) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn event(this: &TailEvent) -> JsValue;
}
