use std::rc::Rc;

use rand::Rng;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{js_sys::Function, Event, HtmlInputElement, KeyboardEvent, MouseEvent, SubmitEvent};
use yew::{
    function_component, html, use_effect_with, use_memo, use_state, Html, NodeRef, Properties,
    UseStateHandle, UseStateSetter,
};

use crate::{
    components::{use_event_on_context, ONKEYDOWN_EVENT_NAME},
    context::State,
    i18n::{Locale, TranslationMap, I18N},
    irregular_verb::GermanVerb,
};

fn check_values_and_modify(
    input: &HtmlInputElement,
    expected_value: &str,
    translations: &Rc<Option<TranslationMap>>,
) -> bool {
    if !input.disabled() {
        let value = input.value();
        let on_valid = |input: &HtmlInputElement| {
            input.set_onchange(None);
            input.set_disabled(true);
            input.set_class_name("table-input-validated");
            input.set_custom_validity("");
        };
        if !value.eq_ignore_ascii_case(expected_value) {
            let onchange: Function = {
                let input = input.clone();
                let expected_value = expected_value.to_string();
                let event = Box::new(move |_: Event| {
                    if input.value().eq_ignore_ascii_case(&expected_value) {
                        on_valid(&input);
                    }
                }) as Box<dyn FnMut(_)>;
                let closure = Closure::wrap(event);
                closure.into_js_value().unchecked_into()
            };
            input.set_onchange(Some(&onchange));
            if let Some(translations) = translations.as_ref() {
                input.set_custom_validity(
                    translations
                        .get("field_error")
                        .unwrap_or(&"field_error".to_string()),
                );
            } else {
                input.set_custom_validity("This is not the expected value");
            }
            false
        } else {
            on_valid(input);
            true
        }
    } else {
        true
    }
}

#[derive(Properties, PartialEq)]
pub struct GameProperties {
    #[prop_or_default]
    pub translations: Rc<Option<TranslationMap>>,
    #[prop_or_default]
    pub locale: Option<Locale>,
    pub state_setter: UseStateSetter<State>,
    pub errors: UseStateHandle<Vec<GermanVerb>>,
    #[prop_or_default]
    pub verbs: Rc<Vec<GermanVerb>>,
}

#[function_component(Game)]
pub fn game(props: &GameProperties) -> Html {
    let translations = &props.translations;
    let locale = &props.locale;
    let default_locale = Locale::get_default_locale();
    let state_setter = &props.state_setter;
    let verbs = &props.verbs;
    let errors = &props.errors;

    let (infinitiv_ref, prasens_ich_ref, prasens_er_ref, preterit_ref, partizip_ii_ref) = (
        NodeRef::default(),
        NodeRef::default(),
        NodeRef::default(),
        NodeRef::default(),
        NodeRef::default(),
    );

    let index = use_state(|| 0);

    let index_val = *index;
    let errors_val: Vec<GermanVerb> = (*errors).to_vec();

    let given_value = use_memo(index_val, |_| rand::thread_rng().gen_range(0u8..5u8));

    let number_of_verbs = verbs.len();

    let verb = use_memo(index_val, move |index| verbs.get(*index).cloned());
    let Some(verb) = verb.as_ref() else {
        state_setter.set(State::Welcome);
        return html! {<></>};
    };
    let meaning = if let Some(locale) = locale {
        verb.meaning.get(&locale.short_name)
    } else if let Some(default_locale) = default_locale {
        verb.meaning.get(&default_locale.short_name)
    } else {
        None
    };

    let submit_event = generate_by_cloning! {
        {

        move |_: Event| {
            if let (
                Some(infinitiv_ref),
                Some(prasens_ich_ref),
                Some(prasens_er_ref),
                Some(preterit_ref),
                Some(partizip_ii_ref),
            ) = (
                infinitiv_ref.cast::<HtmlInputElement>(),
                prasens_ich_ref.cast::<HtmlInputElement>(),
                prasens_er_ref.cast::<HtmlInputElement>(),
                preterit_ref.cast::<HtmlInputElement>(),
                partizip_ii_ref.cast::<HtmlInputElement>(),
            ) {
                let mut first_ref_in_error: Option<&HtmlInputElement> = None;
                let all_ref = [
                    &infinitiv_ref,
                    &prasens_ich_ref,
                    &prasens_er_ref,
                    &preterit_ref,
                    &partizip_ii_ref,
                ];

                if !check_values_and_modify(&partizip_ii_ref, &verb.partizip_ii, &translations) {
                    first_ref_in_error = Some(&partizip_ii_ref);
                }
                if !check_values_and_modify(&preterit_ref, &verb.preterit, &translations) {
                    first_ref_in_error = Some(&preterit_ref);
                }
                if !check_values_and_modify(&prasens_er_ref, &verb.prasens_er, &translations) {
                    first_ref_in_error = Some(&prasens_er_ref);
                }
                if !check_values_and_modify(&prasens_ich_ref, &verb.prasens_ich, &translations) {
                    first_ref_in_error = Some(&prasens_ich_ref);
                }
                if !check_values_and_modify(&infinitiv_ref, &verb.infinitiv, &translations) {
                    first_ref_in_error = Some(&infinitiv_ref);
                }

                if let Some(first_ref_in_error) = first_ref_in_error {
                    first_ref_in_error.focus().unwrap();
                    first_ref_in_error.report_validity();
                    if !(*errors).to_vec().contains(&verb) {
                        errors.set(
                            (*errors)
                                .to_vec()
                                .iter()
                                .chain([verb.clone()].iter())
                                .cloned()
                                .collect(),
                        );
                    }
                } else {
                    all_ref.iter().for_each(|html_ref| {
                        html_ref.set_value("");
                        html_ref.set_disabled(false);
                        html_ref.set_class_name("table-input");
                        html_ref.set_onchange(None);
                        html_ref.set_custom_validity("");
                    });
                    let next_index = *index + 1;
                    if next_index < number_of_verbs {
                        index.set(*index + 1);
                    } else {
                        state_setter.set(State::End);
                    }
                }
            }
        }}
    , infinitiv_ref, prasens_ich_ref, prasens_er_ref, preterit_ref, partizip_ii_ref, verb, state_setter, translations, errors, index};

    use_event_on_context(
        {
            move |keydown: KeyboardEvent| {
                let insertable = if keydown.alt_key() {
                    match keydown.key_code() {
                        85 => Some('ü'),
                        83 => Some('ß'),
                        73 => Some('ï'),
                        65 => Some('ä'),
                        _ => None,
                    }
                } else {
                    None
                };
                if let Some(insertable) = insertable {
                    keydown.prevent_default();
                    if let Some(target) = keydown.target() {
                        if let Some(input) = target.dyn_ref::<HtmlInputElement>() {
                            let input_val = input.value();
                            input.set_value(&format!("{input_val}{insertable}"));
                        }
                    }
                };
            }
        },
        ONKEYDOWN_EVENT_NAME,
    );

    let clear_inputs = create_callback_with_local_clone! {

            if let (
                Some(infinitiv_ref),
                Some(prasens_ich_ref),
                Some(prasens_er_ref),
                Some(preterit_ref),
                Some(partizip_ii_ref),
            ) = (
                infinitiv_ref.cast::<HtmlInputElement>(),
                prasens_ich_ref.cast::<HtmlInputElement>(),
                prasens_er_ref.cast::<HtmlInputElement>(),
                preterit_ref.cast::<HtmlInputElement>(),
                partizip_ii_ref.cast::<HtmlInputElement>(),
            ) {
                let all_ref = [
                    &infinitiv_ref,
                    &prasens_ich_ref,
                    &prasens_er_ref,
                    &preterit_ref,
                    &partizip_ii_ref,
                ];

                for html_ref in all_ref {
                    if !html_ref.disabled() {
                        html_ref.set_value("");
                    }
                }
            }

    , infinitiv_ref, prasens_ich_ref, prasens_er_ref, preterit_ref, partizip_ii_ref};

    let stop_here = create_callback_with_local_clone! {
            state_setter.set(State::End)
    , state_setter};

    let onsubmit = generate_by_cloning! {
        move |e: SubmitEvent| {
            submit_event(e.into());
        }
    , submit_event};

    let onvalidate = generate_by_cloning! {
        move |e: MouseEvent| {
            submit_event(e.into());
        }
    , submit_event};

    let giveup = create_callback_with_local_clone! {
            if let (
                Some(infinitiv_ref),
                Some(prasens_ich_ref),
                Some(prasens_er_ref),
                Some(preterit_ref),
                Some(partizip_ii_ref),
            ) = (
                infinitiv_ref.cast::<HtmlInputElement>(),
                prasens_ich_ref.cast::<HtmlInputElement>(),
                prasens_er_ref.cast::<HtmlInputElement>(),
                preterit_ref.cast::<HtmlInputElement>(),
                partizip_ii_ref.cast::<HtmlInputElement>(),
            ) {
                let all_ref = [
                    &infinitiv_ref,
                    &prasens_ich_ref,
                    &prasens_er_ref,
                    &preterit_ref,
                    &partizip_ii_ref,
                ];
                all_ref.iter().for_each(|html_ref| {
                    html_ref.set_value("");
                    html_ref.set_disabled(false);
                    html_ref.set_class_name("table-input");
                    html_ref.set_onchange(None);
                    html_ref.set_custom_validity("");
                });
                let next_index = *index + 1;
                if !(*errors).to_vec().contains(&verb) {
                    errors.set(
                        (*errors)
                            .to_vec()
                            .iter()
                            .chain([verb.clone()].iter())
                            .cloned()
                            .collect(),
                    );
                }
                if next_index < number_of_verbs {
                    index.set(*index + 1);
                } else {
                    state_setter.set(State::End);
                }
            }
    , infinitiv_ref, prasens_ich_ref, prasens_er_ref, preterit_ref, partizip_ii_ref, state_setter, errors, index, verb};

    {
        let (
            infinitiv_ref,
            prasens_ich_ref,
            prasens_er_ref,
            preterit_ref,
            partizip_ii_ref,
            given_value,
            verb,
        ) = (
            infinitiv_ref.clone(),
            prasens_ich_ref.clone(),
            prasens_er_ref.clone(),
            preterit_ref.clone(),
            partizip_ii_ref.clone(),
            given_value.clone(),
            verb.clone(),
        );
        use_effect_with(*index, move |_| {
            if let (
                Some(infinitiv_ref),
                Some(prasens_ich_ref),
                Some(prasens_er_ref),
                Some(preterit_ref),
                Some(partizip_ii_ref),
            ) = (
                infinitiv_ref.cast::<HtmlInputElement>(),
                prasens_ich_ref.cast::<HtmlInputElement>(),
                prasens_er_ref.cast::<HtmlInputElement>(),
                preterit_ref.cast::<HtmlInputElement>(),
                partizip_ii_ref.cast::<HtmlInputElement>(),
            ) {
                if *given_value != 0 {
                    let _ = infinitiv_ref.focus();
                } else {
                    let _ = prasens_ich_ref.focus();
                }
                let input: (HtmlInputElement, &str) = match *given_value {
                    1 => (prasens_ich_ref, &verb.prasens_ich),
                    2 => (prasens_er_ref, &verb.prasens_er),
                    3 => (preterit_ref, &verb.preterit),
                    4 => (partizip_ii_ref, &verb.partizip_ii),
                    _ => (infinitiv_ref, &verb.infinitiv),
                };
                input.0.set_disabled(true);
                input.0.set_value(input.1);
            }
        });
    }

    html! {
        <div class="flex flex-col space-y-4 justify-between items-center h-full w-full overflow-visible">

            <div>
                if let Some(meaning) = meaning {
                    <h1 class="text-2xl md:text-4xl font-bold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-pink-600 dark:from-pink-500 to-violet-700 dark:to-violet-500">{format!("{meaning} ({}/{})", index_val+1, number_of_verbs)}</h1>
                }
            </div>

            <form>
            <div class="flex flex-row md:flex-col border-separate space-x-2 md:space-x-0 md:space-y-4 w-full h-full">
                <div class="flex flex-col md:flex-row justify-evenly w-full text-center">
                    <p class="basis-1/5">{"Infinitiv    "}</p>
                    <p class="basis-1/5">{"Präsens (ich)"}</p>
                    <p class="basis-1/5">{"Präsens (er) "}</p>
                    <p class="basis-1/5">{"Preterit     "}</p>
                    <p class="basis-1/5">{"Partizip II  "}</p>
                </div>
                <form class="flex flex-col md:flex-row md:space-x-2 justify-evenly w-full" method="POST" action="javascript:void(0);" {onsubmit}>
                    <input autocomplete="off" ref={infinitiv_ref} required=true spellcheck="false" type="text" name="infinitiv" placeholder=" " class="table-input"/>
                    <input autocomplete="off" ref={prasens_ich_ref} required=true spellcheck="false" type="text" name="prasens_ich" placeholder=" " class="table-input"/>
                    <input autocomplete="off" ref={prasens_er_ref} required=true spellcheck="false" type="text" name="prasens_er" placeholder=" " class="table-input"/>
                    <input autocomplete="off" ref={preterit_ref} required=true spellcheck="false" type="text" name="preterit" placeholder=" "  class="table-input"/>
                    <input autocomplete="off" ref={partizip_ii_ref} required=true spellcheck="false" type="text" name="partizip_ii" placeholder=" " class="table-input"/>
                    <input type="submit" hidden=true />
                </form>
            </div>

            </form>

            <div class="flex flex-col w-full space-y-2">

                if !errors_val.is_empty() {
                    <p>
                        <I18N label={"error_number"} {translations}/> {" : "} {errors_val.len()}
                    </p>
                }

                <div class="flex flex-row-reverse space-x-3 space-x-reverse items-center justify-center text-xs sm:text-sm md:text-base">
                    <button onclick={onvalidate} class="btn btn-success flex-1" >
                        <I18N label={"validate"} {translations}/>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                        </svg>
                    </button>
                    <button onclick={clear_inputs} class="btn btn-neutral flex-1" >
                        <span><I18N label={"clear_inputs"} {translations}/></span>
                    </button>

                </div>
                <div class="flex space-x-3 items-center justify-center  text-xs sm:text-sm md:text-base">
                    <button onclick={giveup} class="btn btn-warning flex-1" >
                        <span><I18N label={"give_up"} {translations}/></span>
                    </button>
                    <button onclick={stop_here} class="btn btn-error flex-1" >
                        <span><I18N label={"stop_here"} {translations}/></span>
                    </button>
                </div>
            </div>
        </div>
    }
}
