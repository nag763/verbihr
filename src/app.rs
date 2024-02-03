use std::rc::Rc;

use rand::seq::SliceRandom;
use yew::{
    classes, function_component, html, use_memo, use_state, ContextProvider, Html, UseStateHandle,
};

use crate::components::prelude::*;
use crate::context::{Context, State};
use crate::i18n::Locale;
use crate::irregular_verb::GermanVerb;
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

    let dark_mode = use_state(|| {
        if let Ok(val) = web_sys::window()
            .unwrap()
            .match_media("(prefers-color-scheme: dark)")
        {
            val.is_some()
        } else {
            false
        }
    });

    let dark_mode_val = *dark_mode;

    let state = use_state(State::default);

    let context = Rc::new(Context {
        locale,
        dark_mode,
        translations,
        state,
        errors: use_state(Vec::new),
        verbs: {
            let mut verbs = GermanVerb::get_verbs();
            verbs.shuffle(&mut rand::thread_rng());
            Rc::new(verbs)
        },
    });

    html! {
        <ContextProvider <Rc<Context>> {context}>
            <div class={classes!( dark_mode_val.then_some("dark"))}>
                <div class={classes!["h-screen", "grid", "grid-rows-12", "auto-rows-fr"]}>
                    <div class="row-span-1 bg-gradient-to-r from-black via-red-700 to-yellow-500">
                            <Header/>
                        </div>
                        <div class="row-span-10 bg-slate-200 dark:bg-gray-900">
                            <Body />

                        </div>
                        <div class="row-span-1">
                            <Footer/>
                        </div>
                </div>
            </div>
        </ContextProvider<Rc<Context>>>
    }
}
