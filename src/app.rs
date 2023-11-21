use std::rc::Rc;

use yew::{function_component, Html, html, classes, use_memo, use_state, UseStateHandle, ContextProvider};

use crate::components::prelude::*;
use crate::context::Context;
use crate::i18n::Locale;
use crate::utils::get_navigator_languages;

#[function_component(App)]
pub fn app() -> Html {
    let locale : UseStateHandle<Option<Locale>> = use_state(|| {
        if let Some(navigator_language) = get_navigator_languages() {
            Locale::get_locale_for_navigator_languages(navigator_language)
        } else if !Locale::get_locales().is_empty() {
            Locale::get_default_locale()
        } else {
            None
        }
    });

    let translations = use_memo((*locale).clone(), |locale| {
        if let Some(locale) = locale {
            Some(locale.clone().translations)
        } else {
            None
        }
    });
    
    let dark_mode = use_state(|| {
        if let Ok(val) = web_sys::window().unwrap().match_media("(prefers-color-scheme: dark)") {
            val.is_some()
        } else {
            false
        }
    });

    let context = Rc::new(Context {
        locale,
        dark_mode,
        translations
    });

    html! {

        <ContextProvider <Rc<Context>> {context}>
            <div class={classes!["h-screen", "grid", "grid-rows-12", "auto-rows-fr"]}>
            <div class="row-span-1">
                    <Header/>
                </div>
                <div class="row-span-10">
                    <Body />

                </div>
                <div class="row-span-1">
                    <Footer/>
                </div>
            </div>
        </ContextProvider<Rc<Context>>>
    }
}