use std::rc::Rc;

use yew::{function_component, html, Html, Properties, UseStateHandle};

use crate::{
    i18n::{Locale, TranslationMap},
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
}

#[function_component(End)]
pub fn end(props: &EndProperties) -> Html {
    let errors = &props.errors;
    let errors_val = errors.to_vec();
    html! {<>{format!("End, number of errors : {}", errors_val.len())}</>}
}
