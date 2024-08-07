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
      <div class="flex flex-col items-center justify-between h-full w-full">
            <h1 class="text-2xl md:text-4xl font-bold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-pink-600 dark:from-pink-500 to-violet-700 dark:to-violet-500" >{"Willkomen!"}</h1>
            <div class="flex-grow"></div>
            <p class="text-md md:text-lg"><I18N label={"intro"} {translations}/></p>
            <div class="flex-grow"></div>
            <button {onclick}  class="btn btn-primary btn-lg btn-wide animate-pulse animate-twice">
              <span><I18N label={"start"} {translations}/></span>
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
            </button>
            <div class="flex-grow"></div>
          </div>
    }
}

#[function_component(Body)]
pub fn body() -> Html {
    let context = use_context::<Rc<Context>>().unwrap();
    let translations = &context.translations;
    let locale = context.locale.as_ref().cloned();
    let state = use_state(State::default);
    let state_setter = state.setter();
    let verbs = use_memo(*state == State::End, |_| {
        let mut verbs = GermanVerb::get_verbs();
        verbs.shuffle(&mut rand::thread_rng());
        verbs
    });

    let errors = use_state(Vec::<GermanVerb>::new);

    let component: VNode = match *state {
        State::Welcome => {
            html! {<WelcomeBody {translations} onclick={move |_| {state_setter.set(State::Game)}} />}
        }
        State::Game => html! { <Game {translations}  {locale} {state_setter} {verbs} {errors} />},
        State::End => {
            html! { <End {translations} {locale} {state_setter} {errors} verbs={(*verbs).to_vec()}/> }
        }
    };

    html! {
        <main class="grid grid-cols-9 items-center justify-center  p-6 md:p-12 print:p-6 h-full">
            <div class="col-span-1 md:col-span-2 md:print:col-span-1"/>
            <div class="items-center justify-center text-center col-span-7 md:col-span-5 md:print:col-span-7 h-full overflow-y-auto print:overflow-visible" >
            {component}
            </div>
            <div class="col-span-1 md:col-span-2 "/>
        </main>
    }
}
