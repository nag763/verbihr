use yew::{function_component, Html, html};

#[function_component(Body)]
pub fn body() -> Html {
    html! {
        <main class="flex-1 flex items-center justify-center text-black dark:text-white p-6 ">
          <div class="text-center">
            <h1 class="text-4xl font-bold mb-4">{"Willkomen"}</h1>
            <p class="text-lg">{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam hendrerit est eu justo sodales..."}</p>
          </div>
        </main>
    }
}