use wasm_bindgen::JsValue;
// #[cfg(not(target_arch = "wasm32"))]
// pub type Result<T> = std::result::Result<T, anyhow::Error>;
// #[cfg(target_arch = "wasm32")]
pub type Result<T> = std::result::Result<T, JsValue>;