use std::rc::Rc;

use yew::UseStateHandle;

use crate::{
    i18n::{Locale, TranslationMap},
    irregular_verb::GermanVerb,
};

#[derive(Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Welcome,
    Game,
    End,
}

#[derive(Clone, PartialEq)]
pub struct Context {
    pub locale: UseStateHandle<std::option::Option<Locale>>,
    pub dark_mode: UseStateHandle<bool>,
    pub translations: Rc<Option<TranslationMap>>,
    pub errors: UseStateHandle<Vec<GermanVerb>>,
    pub is_modal_open: UseStateHandle<bool>,
}