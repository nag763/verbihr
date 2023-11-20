use yew::{function_component, Html, html};

#[function_component(Body)]
pub fn body() -> Html {
    html! {
        <main class="flex-1 flex items-center justify-center text-white p-6">
          <div class="text-center">
            <h1 class="text-4xl font-bold mb-4 text-black">{"Willkomen"}</h1>
            <p class="text-lg text-black">{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam hendrerit est eu justo sodales..."}</p>
          </div>
        </main>
    }
}