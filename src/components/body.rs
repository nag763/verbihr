use std::rc::Rc;

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{js_sys::Function, window, Event, KeyboardEvent, MouseEvent};
use yew::{
    function_component, html, use_context, use_effect_with, virtual_dom::VNode, Callback, Html,
    Properties,
};

use crate::{
    components::game::Game,
    context::{Context, State},
    i18n::{TranslationMap, I18N},
};

const ONKEYDOWN_EVENT_NAME: &str = "keydown";

#[derive(Properties, PartialEq)]
pub struct WelcomeBodyProps {
    #[prop_or_default]
    pub translations: Rc<Option<TranslationMap>>,
    pub onclick: Callback<Event>,
}

#[function_component(WelcomeBody)]
pub fn welcome_body(props: &WelcomeBodyProps) -> Html {
    let translations = &props.translations;
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |me: MouseEvent| {
            onclick.emit(me.into());
        })
    };

    let onkeydown: Function = {
        let onclick = props.onclick.clone();
        let event = Box::new(move |keydown: KeyboardEvent| {
            keydown.prevent_default();
            if keydown.key_code() == 13 {
                onclick.emit(keydown.into());
            }
        }) as Box<dyn FnMut(_)>;
        let closure = Closure::wrap(event);
        closure.into_js_value().unchecked_into()
    };

    use_effect_with(onkeydown, move |onkeydown| {
        let window = window().unwrap();
        window
            .add_event_listener_with_callback(ONKEYDOWN_EVENT_NAME, onkeydown)
            .unwrap();
        {
            let onkeydown = onkeydown.clone();
            move || {
                window
                    .remove_event_listener_with_callback(ONKEYDOWN_EVENT_NAME, &onkeydown)
                    .unwrap();
            }
        }
    });

    html! {
      <>
            <h1 class="text-2xl md:text-4xl font-bold mb-4" >{"Willkomen!"}</h1>
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

#[function_component(Body)]
pub fn body() -> Html {
    let context = use_context::<Rc<Context>>().unwrap();
    let translations = &context.translations;
    let locale = context.locale.as_ref().cloned();
    let state_setter = context.state.setter();

    let component: VNode = match *context.state {
        State::Welcome => {
            html! {<WelcomeBody {translations} onclick={move |_| {state_setter.set(State::Game)}} />}
        }
        State::Game => html! { <Game {translations}  {locale} {state_setter} />},
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
