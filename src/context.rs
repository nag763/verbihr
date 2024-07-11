use std::rc::Rc;

use yew::UseStateHandle;

use crate::i18n::{Locale, TranslationMap};

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
    pub translations: Rc<Option<TranslationMap>>,
    pub is_modal_open: UseStateHandle<bool>,
}
