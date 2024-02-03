use std::rc::Rc;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{js_sys::Function, window, KeyboardEvent, MouseEvent};
use yew::{
    function_component, html, use_effect_with, Callback, Event, Html, Properties, UseStateHandle,
    UseStateSetter,
};

use crate::{
    context::State,
    i18n::{Locale, TranslationMap, I18N},
    irregular_verb::GermanVerb,
};

#[derive(Properties, PartialEq)]
pub struct EndProperties {
    #[prop_or_default]
    pub translations: Rc<Option<TranslationMap>>,
    #[prop_or_default]
    pub locale: Option<Locale>,
    #[prop_or_default]
    pub verbs: Vec<GermanVerb>,
    pub errors: UseStateHandle<Vec<GermanVerb>>,
    pub state_setter: UseStateSetter<State>,
}

const ONKEYDOWN_EVENT_NAME: &str = "keydown";

#[function_component(End)]
pub fn end(props: &EndProperties) -> Html {
    let errors = &props.errors;
    let errors_setter = errors.clone();
    let errors_val = errors.to_vec();
    let translations = &props.translations;
    let state_setter = props.state_setter.clone();

    let leaveevent = {
        let state_setter = state_setter.clone();
        let errors_setter = errors_setter.clone();
        Callback::from(move |_event: Event| {
            state_setter.set(State::Welcome);
            errors_setter.set(vec![]);
        })
    };

    let onkeydown: Function = {
        let onkeydown = leaveevent.clone();
        let event = Box::new(move |keydown: KeyboardEvent| {
            keydown.prevent_default();
            if keydown.key_code() == 13 {
                onkeydown.emit(keydown.into());
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
            <div class="flex flex-col space-y-4 justify-between h-full">
                <div>
                    <h1 class="text-2xl md:text-4xl font-bold mb-4">
                        <I18N label={"end_reached"} {translations}/>
                    </h1>
                    <h2 class="text-xl md:text-md">
                        if !errors_val.is_empty() {
                            <I18N label={"end_mistakes"} {translations}/>
                        } else {
                            <I18N label={"end_perfect"} {translations}/>
                        }
                    </h2>
                </div>
                if !errors_val.is_empty() {
                    <div>
                        <I18N label={"error_number"} {translations}/> {" : "} {errors_val.len()}
                    </div>
                }

                <div class="flex items-center justify-center">
                    <button onclick={move |e: MouseEvent| leaveevent.emit(e.into())} class="bg-blue-500 text-white py-4 px-8 rounded-lg flex items-center justify-center space-x-2 hover:bg-blue-400 transition duration-300 w-2/3 md:w-1/3 h-1/6" >
                        <span><I18N label={"restart"} {translations}/></span>
                    </button>
                </div>
            </div>
        </>
    }
}
