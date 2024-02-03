use std::rc::Rc;

use yew::UseStateHandle;

use crate::i18n::{Locale, TranslationMap};

#[derive(Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Welcome,
    Game,
}

#[derive(Clone, PartialEq)]
pub struct Context {
    pub locale: UseStateHandle<std::option::Option<Locale>>,
    pub dark_mode: UseStateHandle<bool>,
    pub translations: Rc<Option<TranslationMap>>,
    pub state: UseStateHandle<State>,
}
