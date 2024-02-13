use std::rc::Rc;

use rand::seq::SliceRandom;
use web_sys::{Event, KeyboardEvent, MouseEvent};
use yew::{
    function_component, html, use_context, use_memo, use_state, virtual_dom::VNode, Callback, Html,
    Properties,
};

use crate::{
    components::{end::End, game::Game, use_event_on_context, ONKEYDOWN_EVENT_NAME},
    context::{Context, State},
    i18n::{TranslationMap, I18N},
    irregular_verb::GermanVerb,
};

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

    use_event_on_context(
        {
            let onclick = props.onclick.clone();
            move |keydown: KeyboardEvent| {
                if keydown.key_code() == 13 {
                    keydown.prevent_default();
                    onclick.emit(keydown.into());
                }
            }
        },
        ONKEYDOWN_EVENT_NAME,
    );

    html! {
      <>
            <h1 class="text-2xl md:text-4xl font-bold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-pink-600 dark:from-pink-500 to-violet-700 dark:to-violet-500" >{"Willkomen!"}</h1>
            <div class="flex-grow"></div>
            <p class="text-md md:text-lg"><I18N label={"intro"} {translations}/></p>
            <div class="flex-grow"></div>
            <button {onclick}  class="bg-blue-700 text-white py-4 px-8 rounded-lg flex items-center justify-center space-x-2 hover:bg-blue-500 transition duration-300 w-2/3 md:w-1/3 h-1/6 focus:outline-none focus-visible:scale-105">
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
    let state = use_state(State::default);
    let state_setter = state.setter();
    let errors = context.errors.clone();
    let verbs = use_memo(*state == State::End, |_| {
        let mut verbs = GermanVerb::get_verbs();
        verbs.shuffle(&mut rand::thread_rng());
        verbs
    });

    let component: VNode = match *state {
        State::Welcome => {
            html! {<WelcomeBody {translations} onclick={move |_| {state_setter.set(State::Game)}} />}
        }
        State::Game => html! { <Game {translations}  {locale} {state_setter} {errors} {verbs} />},
        State::End => html! { <End {translations} {locale} {errors} {state_setter} /> },
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
