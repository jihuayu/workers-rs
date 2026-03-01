use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WorkflowBinding;

    #[wasm_bindgen(method, catch)]
    pub fn trigger(this: &WorkflowBinding, input: JsValue) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=getStatus)]
    pub fn get_status(
        this: &WorkflowBinding,
        workflow_id: &str,
    ) -> Result<js_sys::Promise, JsValue>;
}
