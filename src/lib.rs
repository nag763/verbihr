use app::App;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod app;
pub mod components;
pub mod context;
pub mod i18n;
pub mod utils;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
