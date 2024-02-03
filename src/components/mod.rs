pub mod footer {
    use std::rc::Rc;

    use yew::{function_component, html, use_context, Html};

    use crate::{context::Context, i18n::I18N};

    #[function_component(Footer)]
    pub fn footer() -> Html {
        let context = use_context::<Rc<Context>>().unwrap();
        let translations = &context.translations;
        html! {
            <footer class="text-center py-1 lg:py-2 bg-slate-200 dark:bg-gray-900 h-full text-black dark:text-white text-xs h-full ">
            <div class="flex lg:flex-col items-center justify-center space-x-2 mt-auto">
                <a href="https://github.com/nag763/verbihr" class="transform transition-transform duration-300 hover:scale-110" target="_blank">
                    <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path d="M12 0C5.373 0 0 5.373 0 12c0 5.303 3.438 9.8 8.207 11.387.6.11.793-.258.793-.577 0-.285-.012-1.04-.018-2.04-3.22.702-3.89-1.54-3.89-1.54-.525-1.327-1.282-1.68-1.282-1.68-1.048-.715.08-.702.08-.702 1.16.082 1.773 1.2 1.773 1.2 1.033 1.77 2.713 1.258 3.37.96.105-.748.405-1.26.737-1.546-2.586-.294-5.297-1.293-5.297-5.74 0-1.27.45-2.312 1.2-3.126-.12-.296-.522-1.482.114-3.08 0 0 1.008-.312 3.3 1.2a11.115 11.115 0 012.947-.4c1.002.007 2.007.135 2.947.4 2.29-1.512 3.297-1.2 3.297-1.2.636 1.598.234 2.784.114 3.08.75.814 1.2 1.856 1.2 3.126 0 4.458-2.715 5.442-5.305 5.728.42.36.795 1.068.795 2.15 0 1.55-.015 2.8-.015 3.18 0 .318.21.694.8.576C20.568 21.797 24 16.3 24 12c0-6.627-5.373-12-12-12z"/>
                    </svg>
                </a>
                <p>{"© 2023 LABEYE Loïc. "}<I18N label="all_rights_reserved" {translations} /></p>
                <p class="hidden lg:block"><I18N label="footer" {translations} /> </p>
              </div>
            </footer>
        }
    }
}

pub mod header {

    use std::rc::Rc;

    use web_sys::{Event, HtmlSelectElement, MouseEvent};
    use yew::{function_component, html, use_context, Callback, Html, NodeRef};

    use crate::{
        context::{Context, State},
        i18n::Locale,
    };

    #[function_component(Header)]
    pub fn header() -> Html {
        let light_mode_icon: Html = html! {
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="5"></circle>
                <line x1="12" y1="1" x2="12" y2="3"></line>
                <line x1="12" y1="21" x2="12" y2="23"></line>
                <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                <line x1="1" y1="12" x2="3" y2="12"></line>
                <line x1="21" y1="12" x2="23" y2="12"></line>
                <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
            </svg>
        };

        let dark_mode_icon: Html = html! {
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z"></path>
            </svg>

        };

        let context = use_context::<Rc<Context>>().unwrap();
        let state_setter = context.state.setter();

        let user_locale = &context.locale;
        let select_ref = NodeRef::default();
        let is_selected = |locale: &Locale| {
            if let Some(user_locale) = user_locale.as_ref() {
                user_locale.short_name == locale.short_name
            } else {
                false
            }
        };
        let onchange = {
            let select_ref = select_ref.clone();
            let user_locale = user_locale.clone();
            Callback::from(move |_e: Event| {
                if let Some(select) = select_ref.cast::<HtmlSelectElement>() {
                    if let Some(locale) = Locale::get_by_short_name(&select.value()) {
                        user_locale.set(Some(locale));
                    }
                }
            })
        };

        let onclick = {
            let dark_mode_enabled = context.dark_mode.clone();
            let dark_mode_val = *context.dark_mode;
            Callback::from(move |_e: MouseEvent| {
                dark_mode_enabled.set(!dark_mode_val);
            })
        };

        html! {
            <nav class="flex items-center justify-between px-2 sm:px-4 md:px-6 h-py-1 sm:py-2 md:py-4 h-full">
            <div class="text-white font-bold text-xl" onclick={move |_| state_setter.set(State::default())}>{"Verbihr"}</div>
            <div class="flex space-x-4">
            <div {onclick} class="w-6 h-6 text-black transform transition-transform duration-300 hover:rotate-180">
                if *context.dark_mode {
                    {dark_mode_icon}
                } else {
                    {light_mode_icon}
                }
            </div>
                <select {onchange} ref={select_ref} class="border-0 bg-transparent hover:text-blue-500 transition-colors duration-300">
                    {Locale::get_locales().iter().map(|locale|
                        html! { <option value={locale.short_name.to_string()} selected={is_selected(locale)}>{locale.short_name.to_string()}</option> }
                    ).collect::<Html>()}
                </select>
            </div>
          </nav>
        }
    }
}

pub mod body;
mod game;

pub mod prelude {
    pub use super::body::Body;
    pub use super::footer::Footer;
    pub use super::header::Header;
}
