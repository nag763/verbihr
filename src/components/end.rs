use std::rc::Rc;

use web_sys::{KeyboardEvent, MouseEvent};
use yew::{
    function_component, html, Callback, Event, Html, Properties, UseStateHandle, UseStateSetter,
};

use crate::{
    components::{use_event_on_context, ONKEYDOWN_EVENT_NAME},
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

#[function_component(End)]
pub fn end(props: &EndProperties) -> Html {
    let errors = &props.errors;
    let errors_setter = errors.clone();
    let errors_val = errors.to_vec();
    let translations = &props.translations;
    let state_setter = props.state_setter.clone();

    let leaveevent = generate_by_cloning! {
        Callback::from(move |_event: Event| {
            state_setter.set(State::Welcome);
            errors_setter.set(vec![]);
        })
    , state_setter};

    use_event_on_context(
        {
            let onkeydown = leaveevent.clone();
            move |keydown: KeyboardEvent| {
                if keydown.key_code() == 13 {
                    keydown.prevent_default();
                    onkeydown.emit(keydown.into());
                }
            }
        },
        ONKEYDOWN_EVENT_NAME,
    );

    html! {
        <div class="grid grid-rows-6 space-y-4 justify-between max-h-full">
            <div class="row-span-1 print:hidden">
                <h1 class="text-2xl md:text-4xl font-bold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-pink-600 dark:from-pink-500 to-violet-700 dark:to-violet-500">
                    <I18N label={"end_reached"} {translations}/>
                </h1>
                <h2 class="text-sm lg:text-xl">
                    if !errors_val.is_empty() {
                        <I18N label={"end_mistakes"} {translations}/>
                    } else {
                        <I18N label={"end_perfect"} {translations}/>
                    }
                </h2>
            </div>
            <div class="row-span-4 print:row-span-6 overflow-y-auto">
            if !errors_val.is_empty() {
                    <h2 class="text-md md:text-xl text-center print:hidden">
                        <I18N label={"error_number"} {translations}/> {" : "} {errors_val.len()}
                    </h2>
                    <div class="table border-separate border-spacing-2 dark:text-white items-center justify-center w-full text-xs sm:text-sm md:text-base">
                        <div class="table-header-group font-bold">
                            <div class="table-cell">{"Infinitiv"}</div>
                            <div class="table-cell">{"Präsens (ich)"}</div>
                            <div class="table-cell">{"Präsens (er)"}</div>
                            <div class="table-cell">{"Preterit   "}</div>
                            <div class="table-cell">{"Partizip II"}</div>
                        </div>
                        <div class="table-row-group overflow-y-scroll space-y-0.5">
                        {errors_val.iter().map(|v| {
                            html!{
                                <div class="table-row">
                                    <div class="table-cell">{&v.infinitiv}</div>
                                    <div class="table-cell">{&v.prasens_ich}</div>
                                    <div class="table-cell">{&v.prasens_er}</div>
                                    <div class="table-cell">{&v.preterit}</div>
                                    <div class="table-cell">{&v.partizip_ii}</div>
                                </div>
                            }
                        }).collect::<Html>()}

                        </div>
                    </div>
            }
            </div>
            <div class="row-span-1 flex items-center justify-center print:hidden">
                <button onclick={move |e: MouseEvent| leaveevent.emit(e.into())} class="bg-blue-500 text-white py-4 px-8 rounded-lg flex items-center justify-center space-x-2 hover:bg-blue-400 transition duration-300 w-2/3 md:w-1/3 h-1/6 focus:outline-none focus-visible:scale-105" >
                    <span><I18N label={"restart"} {translations}/></span>
                </button>
            </div>
        </div>
    }
}
