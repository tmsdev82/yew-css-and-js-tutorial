use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/js_time.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn get_now_date() -> String;
}