use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/js_time.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn get_now_date() -> String;
}

#[wasm_bindgen(module = "/js/my_chart.js")]
extern "C" {
    pub type MyChart;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MyChart;

    #[wasm_bindgen(method)]
    pub fn draw(this: &MyChart, element_id: &str);

    #[wasm_bindgen(method)]
    pub fn update(this: &MyChart, value: i32);
}

