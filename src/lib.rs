use app::App;
use wasm_bindgen::prelude::wasm_bindgen;


mod i18n;
mod components;
mod utils;
mod context;
pub mod app;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}