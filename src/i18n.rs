use std::{cell::Cell, collections::HashMap, ops::Deref, rc::Rc};

use serde::Deserialize;
use yew::{function_component, Html, html, Properties};

thread_local! {
    static LOCALES: Cell<Vec<Locale>> = Cell::new(postcard::from_bytes(include_bytes!("resources/translation.pc")).unwrap());
}

#[derive(Debug,Clone, PartialEq, Eq, Deserialize)]
pub struct TranslationMap(HashMap<String, String>);

impl Deref for TranslationMap {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, PartialEq, Eq, Clone)]
pub struct Locale {
    long_name: String,
    short_name: String,
    navigator_names: Vec<String>,
    is_default: bool,
    pub translations: TranslationMap
}

impl Locale {

    pub fn get_locales() -> Vec<Locale> {
        LOCALES.take()
    }

    pub fn get_locale_for_navigator_languages(web_names: Vec<String>) -> Option<Locale> {
        let locales = LOCALES.take();
        for web_name in web_names {
            if let Some(index) = locales.iter().position(|v| v.navigator_names.contains(&web_name)) {
                return locales.get(index).cloned();
            }
        }
        return None;
    }

    pub fn get_default_locale() -> Option<Locale> {
        LOCALES.take().into_iter().filter(|locale| locale.is_default).collect::<Vec<Locale>>().get(0).cloned()
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct I18NProperties {
    pub label: String,
    #[prop_or_default]
    pub translations: Rc<Option<TranslationMap>>,
    #[prop_or_default]
    pub default: Option<String>
}

#[function_component(I18N)]
pub fn i18n(props: &I18NProperties) -> Html {
     let value = if let Some(translation_map) = props.translations.as_ref() {
        translation_map.get(&props.label).or(props.default.as_ref())
    } else {
        None
    };
    html!{
        <>if let Some(value) = value { {value} } else { {&props.label} } </>
    }
}