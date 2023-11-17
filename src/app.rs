use yew::{function_component, Html, html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <p class="text-xl">{"Hallo Welt!"}</p>
    }
}