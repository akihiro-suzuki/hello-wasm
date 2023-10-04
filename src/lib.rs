use wasm_bindgen::prelude::*;

// rustがjsを呼ぶ
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

// jsがrustを呼ぶ
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
