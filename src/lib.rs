use app::App;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod app;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}