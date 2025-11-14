use wasm_bindgen::prelude::*;

// Пример экспортируемой функции
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {} from WebAssembly!", name)
}
