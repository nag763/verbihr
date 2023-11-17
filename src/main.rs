use app::App;

pub mod app;

fn main() {
    yew::Renderer::<App>::new().render();
}