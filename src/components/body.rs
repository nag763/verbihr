use std::rc::Rc;

use web_sys::MouseEvent;
use yew::{function_component, Html, html, use_context, Properties, Callback, use_state, virtual_dom::VNode};

use crate::{i18n::{I18N, TranslationMap}, context::Context, components::game::Game};


#[derive(Properties, PartialEq)]
pub struct WelcomeBodyProps {
  #[prop_or_default]
  pub translations: Rc<Option<TranslationMap>>,
  pub onclick: Callback<MouseEvent>
}

#[function_component(WelcomeBody)]
pub fn welcome_body(props: &WelcomeBodyProps) -> Html {
  let translations = &props.translations;
  let onclick = props.onclick.clone();

  html! {
    <>
          <h1 class="text-2xl md:text-4xl font-bold mb-4">{"Willkomen!"}</h1>
          <p class="text-md md:text-lg"><I18N label={"intro"} {translations}/></p>
          <div class="flex-grow"></div>
          <button {onclick}  class="bg-blue-500 text-white py-4 px-8 rounded-lg flex items-center justify-center space-x-2 hover:bg-blue-700 transition duration-300 w-2/3 md:w-1/3 h-1/6">
            <span><I18N label={"start"} {translations}/></span>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
            </svg>
          </button>
          <div class="flex-grow"></div>
        </>
  }
}
#[derive(Default)]
enum State {
  #[default]
  Welcome,
  Game
}

#[function_component(Body)]
pub fn body() -> Html {
  let context = use_context::<Rc<Context>>().unwrap();
  let translations = &context.translations;

  let state = use_state(|| State::default());

  let component: VNode = match *state {
    State::Welcome => html! {<WelcomeBody {translations} onclick={move |_| {state.set(State::Game)}} />},
    State::Game => html! { <Game {translations} />}
  };

  html! {
      <main class="grid grid-cols-9 items-center justify-center text-black dark:text-white p-6 md:p-12 h-full">
      <div class="col-span-1 md:col-span-2"/>
      <div class="flex flex-col items-center justify-center text-center col-span-7 md:col-span-5 h-full">
        {component}
      </div>
      <div class="col-span-1 md:col-span-2"/>
      </main>
  }
}