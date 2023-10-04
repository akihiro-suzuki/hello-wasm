use rand::Rng;
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

#[wasm_bindgen]
pub fn calc(a: i32, b: i32) -> i32 {
    let mut r = rand::thread_rng();
    let random: i32 = r.gen();
    (a + b) * random
}
#[wasm_bindgen]
pub fn random() -> i32 {
    let mut r = rand::thread_rng();
    let random: i32 = r.gen();
    return random;
}
