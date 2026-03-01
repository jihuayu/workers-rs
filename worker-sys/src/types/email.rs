use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type EmailMessage;

    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type SendEmail;

    #[wasm_bindgen(method, getter)]
    pub fn from(this: &EmailMessage) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn to(this: &EmailMessage) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn raw(this: &EmailMessage) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &EmailMessage) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = rawSize)]
    pub fn raw_size(this: &EmailMessage) -> Option<f64>;

    #[wasm_bindgen(method, catch, js_name = setReject)]
    pub fn set_reject(this: &EmailMessage, reason: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn forward(
        this: &EmailMessage,
        rcpt_to: &str,
        headers: JsValue,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn reply(
        this: &EmailMessage,
        message: &EmailMessage,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn send(this: &SendEmail, message: &EmailMessage) -> Result<js_sys::Promise, JsValue>;
}
