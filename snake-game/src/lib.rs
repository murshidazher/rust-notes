use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name);
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str); // external fn same fn name as in the javascript
}

// wasm-pack build --target web
