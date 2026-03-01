use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type DynamicWorkerLoader;

    #[wasm_bindgen(method, catch)]
    pub fn load(
        this: &DynamicWorkerLoader,
        worker_name: &str,
    ) -> Result<js_sys::Promise, JsValue>;
}
