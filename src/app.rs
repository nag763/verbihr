use std::rc::Rc;

use yew::{
    classes, function_component, html, use_memo, use_state, ContextProvider, Html, UseStateHandle,
};

use crate::components::modal::Modal;
use crate::components::prelude::*;
use crate::context::Context;
use crate::i18n::Locale;
use crate::utils::get_navigator_languages;

#[function_component(App)]
pub fn app() -> Html {
    let locale: UseStateHandle<Option<Locale>> = use_state(|| {
        if let Some(navigator_language) = get_navigator_languages() {
            Locale::get_locale_for_navigator_languages(navigator_language)
        } else if !Locale::get_locales().is_empty() {
            Locale::get_default_locale()
        } else {
            None
        }
    });

    let translations = use_memo((*locale).clone(), |locale| {
        locale.as_ref().map(|locale| locale.clone().translations)
    });

    let context = Rc::new(Context {
        locale,
        translations,
        is_modal_open: use_state(|| false),
    });

    html! {
        <ContextProvider <Rc<Context>> {context}>
            <div class={classes!("static", "w-full", "h-screen")}>
                <div class={classes!["h-full", "grid", "grid-rows-12", "auto-rows-fr", "w-full"]}>
                    <div class="row-span-1 bg-gradient-to-r from-black via-red-700 to-yellow-500">
                            <Header/>
                        </div>
                        <div class="row-span-10 bg-opacity-25">
                            <Body />

                        </div>
                        <div class="row-span-1">
                            <Footer/>
                        </div>
                </div>
                <Modal />
            </div>
        </ContextProvider<Rc<Context>>>
    }
}
