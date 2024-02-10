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
    , state_setter, errors_setter};

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
        <>
            <div class="flex flex-col max-h-full space-y-4 justify-between h-full">
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
                    <div class="flex flex-col max-h-full items-center justify-center dark:text-white ">

                        <h2 class="text-xl md:text-md">
                            <I18N label={"error_number"} {translations}/> {" : "} {errors_val.len()}
                        </h2>
                        <div class="max-h-full">
                            <table class="border-separate border-spacing-2 dark:text-white">
                                <thead>
                                    <th>{"Infinitiv"}</th>
                                    <th>{"Präsens (ich)"}</th>
                                    <th>{"Präsens (er)"}</th>
                                    <th>{"Preterit   "}</th>
                                    <th>{"Partizip II"}</th>
                                </thead>
                                <tbody>
                                {errors_val.iter().map(|v| {
                                    html!{
                                        <tr>
                                            <td>{&v.infinitiv}</td>
                                            <td>{&v.prasens_ich}</td>
                                            <td>{&v.prasens_er}</td>
                                            <td>{&v.preterit}</td>
                                            <td>{&v.partizip_ii}</td>
                                        </tr>
                                    }
                                }).collect::<Html>()}

                                </tbody>
                            </table>
                            </div>
                        </div>
                }

                <div class="flex items-center justify-center">
                    <button onclick={move |e: MouseEvent| leaveevent.emit(e.into())} class="bg-blue-500 text-white py-4 px-8 rounded-lg flex items-center justify-center space-x-2 hover:bg-blue-400 transition duration-300 w-2/3 md:w-1/3 h-1/6 focus:outline-none focus-visible:scale-105" >
                        <span><I18N label={"restart"} {translations}/></span>
                    </button>
                </div>
            </div>
        </>
    }
}
