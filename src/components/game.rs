use std::rc::Rc;

use rand::{seq::SliceRandom, Rng};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{js_sys::Function, Event, HtmlInputElement};
use yew::{
    function_component, html, use_effect_with, use_memo, use_state, Html, NodeRef, Properties,
    UseStateSetter,
};

use crate::{
    context::State,
    i18n::{Locale, TranslationMap, I18N},
    irregular_verb::GermanVerb,
};

fn check_values_and_modify(input: &HtmlInputElement, expected_value: &str) -> bool {
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
            input.set_custom_validity(&format!("Not good expected {expected_value}"));
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
}

#[function_component(Game)]
pub fn game(props: &GameProperties) -> Html {
    let translations = &props.translations;
    let locale = &props.locale;
    let default_locale = Locale::get_default_locale();
    let state_setter = &props.state_setter;

    let (infinitiv_ref, prasens_ich_ref, prasens_er_ref, preterit_ref, partizip_ii_ref) = (
        NodeRef::default(),
        NodeRef::default(),
        NodeRef::default(),
        NodeRef::default(),
        NodeRef::default(),
    );

    let index = use_state(|| 0);
    let index_val = *index;
    let given_value = use_state(|| rand::thread_rng().gen_range(0u8..5u8));

    let displayed_field = *given_value;

    let focus_input = {
        let (infinitiv_ref, prasens_ich_ref) = (infinitiv_ref.clone(), prasens_ich_ref.clone());
        move |index: u8| {
            if let (Some(infinitiv_ref), Some(prasens_ich_ref)) = (
                infinitiv_ref.cast::<HtmlInputElement>(),
                prasens_ich_ref.cast::<HtmlInputElement>(),
            ) {
                if index != 0 {
                    let _ = infinitiv_ref.focus();
                } else {
                    let _ = prasens_ich_ref.focus();
                }
            }
        }
    };

    let verbs = {
        let mut verbs = GermanVerb::get_verbs();
        verbs.shuffle(&mut rand::thread_rng());
        verbs
    };
    let number_of_verbs = verbs.len();

    let verb = use_memo((verbs, *index), |(verbs, index)| verbs.get(*index).cloned());
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

    {
        let focus_input = focus_input.clone();
        use_effect_with(displayed_field, move |_| {
            focus_input(displayed_field);
        });
    }

    let onsubmit = {
        let (infinitiv_ref, prasens_ich_ref, prasens_er_ref, preterit_ref, partizip_ii_ref) = (
            infinitiv_ref.clone(),
            prasens_ich_ref.clone(),
            prasens_er_ref.clone(),
            preterit_ref.clone(),
            partizip_ii_ref.clone(),
        );
        let verb = verb.clone();
        let state_setter = state_setter.clone();
        move |_| {
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
                gloo_console::log!("Submit recorded");
                let mut first_ref_in_error: Option<&HtmlInputElement> = None;
                let all_ref = [
                    &infinitiv_ref,
                    &prasens_ich_ref,
                    &prasens_er_ref,
                    &preterit_ref,
                    &partizip_ii_ref,
                ];

                if !check_values_and_modify(&partizip_ii_ref, &verb.partizip_ii) {
                    first_ref_in_error = Some(&partizip_ii_ref);
                }
                if !check_values_and_modify(&preterit_ref, &verb.preterit) {
                    first_ref_in_error = Some(&preterit_ref);
                }
                if !check_values_and_modify(&prasens_er_ref, &verb.prasens_er) {
                    first_ref_in_error = Some(&prasens_er_ref);
                }
                if !check_values_and_modify(&prasens_ich_ref, &verb.prasens_ich) {
                    first_ref_in_error = Some(&prasens_ich_ref);
                }
                if !check_values_and_modify(&infinitiv_ref, &verb.infinitiv) {
                    first_ref_in_error = Some(&infinitiv_ref);
                }

                if let Some(first_ref_in_error) = first_ref_in_error {
                    first_ref_in_error.focus().unwrap();
                    first_ref_in_error.report_validity();
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
                        given_value.set(rand::thread_rng().gen_range(0u8..5u8));
                        focus_input(*given_value);
                    } else {
                        state_setter.set(State::Welcome);
                    }
                }
            }
        }
    };

    html! {
    <>
        <div class="flex flex-col space-y-4 justify-between h-full">

            <div>
                if let Some(meaning) = meaning {
                    <h1 class="text-2xl md:text-4xl font-bold mb-4">{format!("{meaning} ({}/{})", index_val+1, number_of_verbs)}</h1>
                }
            </div>
            <form>
            <div class="border-separate  space-x-2  dark:text-white w-full h-full">
                <div class="flex justify-evenly w-full text-center">
                    <p>{"Infinitiv"}</p>
                    <p>{"Präsens (ich)"}</p>
                    <p>{"Präsens (er)"}</p>
                    <p>{"Preterit   "}</p>
                    <p>{"Partizip II"}</p>
                </div>
                <form class="flex space-x-2 justify-evenly w-full" action="javascript:void(0);" {onsubmit}>
                    <input autocomplete="off" ref={infinitiv_ref} required=true disabled={displayed_field == 0} type="text" name="infinitiv" placeholder=" " value={(displayed_field == 0).then(|| verb.infinitiv.clone())} class="table-input"/>
                    <input autocomplete="off" ref={prasens_ich_ref} required=true disabled={displayed_field == 1} type="text" name="prasens_ich" placeholder=" " value={(displayed_field == 1).then(|| verb.prasens_ich.clone())} autofocus={displayed_field != 0} class="table-input"/>
                    <input autocomplete="off" ref={prasens_er_ref} required=true disabled={displayed_field == 2} type="text" name="prasens_er" placeholder=" " value={(displayed_field == 2).then(|| verb.prasens_er.clone())} class="table-input"/>
                    <input autocomplete="off" ref={preterit_ref} required=true disabled={displayed_field == 3} type="text" name="preterit" placeholder=" " value={(displayed_field == 3).then(|| verb.preterit.clone())} class="table-input"/>
                    <input autocomplete="off" ref={partizip_ii_ref} required=true disabled={displayed_field == 4}  type="text" name="partizip_ii" placeholder=" " value={(displayed_field == 4).then(|| verb.partizip_ii.clone())} class="table-input"/>
                    <input type="submit" hidden=true />
                </form>
            </div>

            </form>

            <div class="flex items-center justify-center">
            <button class="bg-green-600 text-white py-4 px-8 rounded-lg flex items-center justify-center space-x-2 hover:bg-green-500 transition duration-300 w-2/3 md:w-1/3 h-1/6" >
                <span><I18N label={"validate"} {translations}/></span>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                </svg>
            </button>
            </div>
        </div>
    </>
    }
}
