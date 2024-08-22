use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// #[wasm_bindgen]
// pub fn log(name: &str) {
//     log(&format!("Hello, {}!", name));
// }
