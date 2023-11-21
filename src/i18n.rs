use std::{collections::HashMap, ops::Deref, rc::Rc, sync::OnceLock};

use serde::Deserialize;
use yew::{function_component, Html, html, Properties};

static LOCALES: OnceLock<Vec<Locale>> = OnceLock::new();

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
    pub long_name: String,
    pub short_name: String,
    navigator_names: Vec<String>,
    is_default: bool,
    pub translations: TranslationMap
}

impl Locale {

    fn init() -> Vec<Locale> {
        postcard::from_bytes(include_bytes!("resources/translation.pc")).unwrap()
    }

    pub fn get_locales() -> Vec<Locale> {
        LOCALES.get_or_init(Self::init).clone()
    }

    pub fn get_locale_for_navigator_languages(web_names: Vec<String>) -> Option<Locale> {
        let locales = LOCALES.get_or_init(Self::init);
        for web_name in web_names {
            if let Some(index) = locales.iter().position(|v| v.navigator_names.contains(&web_name)) {
                return locales.get(index).cloned();
            }
        }
        return None;
    }

    pub fn get_default_locale() -> Option<Locale> {
        LOCALES.get_or_init(Self::init).into_iter().find(|locale| locale.is_default).cloned()
    }

    pub fn get_by_short_name(short_name: &str) -> Option<Locale> {
        LOCALES.get_or_init(Self::init).into_iter().find(|locale| locale.short_name == short_name).cloned()
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