use wasm_bindgen::prelude::*;

extern crate wasm_bindgen;

mod ai;
mod app;
mod entity;
mod level;
mod player;
mod projectile;
mod ui;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn _console_log(s: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (_console_log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn run_app() {
    console_log!("Start app!");
    app::create_app();
    console_log!("End app!");
}
