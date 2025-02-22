use wasm_bindgen::convert::FromWasmAbi;
use yew::hook;

macro_rules! generate_by_cloning {
    ($b:stmt,  $( $x:ident ),* ) => {
        {
            let ($($x ,)*) = ($($x.clone(),)*);
            $b
        }
    };
}

macro_rules! create_callback {
    ($b:expr ) => {
        yew::Callback::from(move |_e| $b)
    };
}

macro_rules! create_callback_with_local_clone {
    ($b:expr, $( $x:ident ),*  ) => {
        generate_by_cloning!(create_callback!($b), $($x ),*)
    };
}

#[hook]
pub fn use_event_on_context<F, T>(callback: F, trigger: &str)
where
    F: FnMut(T) + 'static,
    T: FromWasmAbi + 'static,
{
    use wasm_bindgen::JsCast;
    let trigger = trigger.to_string();
    let event: web_sys::js_sys::Function = {
        let event = Box::new(callback) as Box<dyn FnMut(_)>;
        let closure = wasm_bindgen::closure::Closure::wrap(event);
        closure.into_js_value().unchecked_into()
    };
    yew::use_effect_with((), move |_| {
        let window = web_sys::window().unwrap();
        window
            .add_event_listener_with_callback(&trigger, &event)
            .unwrap();
        move || {
            window
                .remove_event_listener_with_callback(&trigger, &event)
                .unwrap();
        }
    });
}

pub const ONKEYDOWN_EVENT_NAME: &str = "keydown";

pub mod footer {
    use std::rc::Rc;

    use yew::{function_component, html, use_context, Html};

    use crate::{context::Context, i18n::I18N};

    #[function_component(Footer)]
    pub fn footer() -> Html {
        let context = use_context::<Rc<Context>>().unwrap();
        let translations = &context.translations;
        html! {
            <footer class="text-center py-1 lg:py-2  h-full text-xs h-full print:hidden ">
            <div class="flex lg:flex-col items-center justify-center space-x-2 mt-auto">
                <a href="https://github.com/nag763/verbihr" class="transform transition-transform duration-300 hover:scale-110 focus:outline-none focus-visible:scale-125" target="_blank" >
                    <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path d="M12 0C5.373 0 0 5.373 0 12c0 5.303 3.438 9.8 8.207 11.387.6.11.793-.258.793-.577 0-.285-.012-1.04-.018-2.04-3.22.702-3.89-1.54-3.89-1.54-.525-1.327-1.282-1.68-1.282-1.68-1.048-.715.08-.702.08-.702 1.16.082 1.773 1.2 1.773 1.2 1.033 1.77 2.713 1.258 3.37.96.105-.748.405-1.26.737-1.546-2.586-.294-5.297-1.293-5.297-5.74 0-1.27.45-2.312 1.2-3.126-.12-.296-.522-1.482.114-3.08 0 0 1.008-.312 3.3 1.2a11.115 11.115 0 012.947-.4c1.002.007 2.007.135 2.947.4 2.29-1.512 3.297-1.2 3.297-1.2.636 1.598.234 2.784.114 3.08.75.814 1.2 1.856 1.2 3.126 0 4.458-2.715 5.442-5.305 5.728.42.36.795 1.068.795 2.15 0 1.55-.015 2.8-.015 3.18 0 .318.21.694.8.576C20.568 21.797 24 16.3 24 12c0-6.627-5.373-12-12-12z"/>
                    </svg>
                </a>
                <p>{"© 2023-2024 LABEYE Loïc. "}<I18N label="all_rights_reserved" {translations} /></p>
                <p class="hidden lg:block"><I18N label="footer" {translations} /> </p>
              </div>
            </footer>
        }
    }
}

pub mod header {

    use std::rc::Rc;

    use web_sys::{HtmlInputElement, HtmlSelectElement};
    use yew::{function_component, html, use_context, use_effect_with, Html, NodeRef};

    use crate::{context::Context, i18n::Locale};

    #[function_component(Header)]
    pub fn header() -> Html {
        let theme_ref = NodeRef::default();

        {
            let theme_ref = theme_ref.clone();
            use_effect_with((), move |_| {
                if let Some(theme_ref) = theme_ref.cast::<HtmlInputElement>() {
                    let val = match web_sys::window()
                        .unwrap()
                        .match_media("(prefers-color-scheme: dark)")
                        .unwrap()
                        .unwrap()
                        .matches()
                    {
                        true => "light",
                        false => "dark",
                    };
                    theme_ref.set_value(val);
                }
            });
        }

        let context = use_context::<Rc<Context>>().unwrap();

        let user_locale = &context.locale;
        let is_modal_open = &context.is_modal_open;
        let select_ref = NodeRef::default();
        let is_selected = |locale: &Locale| {
            if let Some(user_locale) = user_locale.as_ref() {
                user_locale.short_name == locale.short_name
            } else {
                false
            }
        };
        let onchange = create_callback_with_local_clone!(
            if let Some(select) = select_ref.cast::<HtmlSelectElement>() {
                if let Some(locale) = Locale::get_by_short_name(&select.value()) {
                    user_locale.set(Some(locale));
                }
            },
            select_ref,
            user_locale
        );

        let oninfoclick =
            create_callback_with_local_clone!(is_modal_open.set(!(*is_modal_open)), is_modal_open);

        html! {
            <nav class="flex items-center justify-between px-2 sm:px-4 md:px-6 h-py-1 sm:py-2 md:py-4 h-full">
            <div class="text-white font-bold text-xl flex space-x-2">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="w-6 h-6">
                    <g data-name="Brandenburg Gate">
                    <path d="M490.667 42.667H21.333V64s21.334 0 21.334 21.333V192h426.666V85.333C469.333 64 490.667 64 490.667 64z" style="fill:#885400"/>
                    <path d="M405.333 64V42.667h-384V64s21.334 0 21.334 21.333V192H384V85.333C384 64 405.333 64 405.333 64z" style="fill:#b37900"/><path style="fill:#b37900" d="M21.333 42.667h469.333V64H21.333z"/>
                    <path style="fill:#d6a436" d="M21.333 42.667h384V64h-384z"/><path style="fill:#cb8c01" d="M426.667 106.667V85.333H384V64H128v21.333H85.333v21.334H42.667V128h426.666v-21.333h-42.666z"/>
                    <path style="fill:#b37900" d="M170.667 64h170.667v64H170.667z"/><path d="M362.667 21.333H149.333v21.334s21.334 0 21.334 21.333v42.667h170.666V64c0-21.333 21.334-21.333 21.334-21.333z" style="fill:#e0bb69"/>
                    <path style="fill:#b37900" d="M21.333 128h469.333v64H21.333z"/><path style="fill:#9e6611" d="M490.667 170.667H21.333V192l21.334 21.333h426.666L490.667 192v-21.333z"/>
                    <path style="fill:#b37900" d="M42.667 192h426.667v85.333H42.667z"/><path d="M62.219 234.667h3.562C69.333 245.396 78.521 256 96 256s26.667-10.604 30.219-21.333h3.562C133.333 245.396 142.521 256 160 256s26.667-10.604 30.219-21.333h3.562C197.333 245.396 206.521 256 224 256s26.667-10.604 30.219-21.333h3.562C261.333 245.396 270.521 256 288 256s26.667-10.604 30.219-21.333h3.562C325.333 245.396 334.521 256 352 256s26.667-10.604 30.219-21.333h3.562C389.333 245.396 398.521 256 416 256s26.667-10.604 30.219-21.333h3.562c2.736 8.261 8.92 16.325 19.552 19.655v-40.989l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792s-10.542-5.875-10.667-10.667v-10.667l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792s-10.542-5.875-10.667-10.667v-10.667l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792s-10.542-5.875-10.667-10.667v-10.667l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792s-10.542-5.875-10.667-10.667v-10.667l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792s-10.542-5.875-10.667-10.667v-10.667l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792S85.458 228.792 85.333 224v-10.667l-42.406.146-.26 10.396v30.447c10.632-3.33 16.816-11.394 19.552-19.655z" style="fill:#9e6611"/>
                    <path d="M62.219 213.333h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c2.736 8.262 8.92 16.326 19.552 19.655V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396v30.446c10.632-3.33 16.816-11.393 19.552-19.655z" style="fill:#cb8c01"/><path style="fill:#d6a436" d="M21.333 128H384v42.667H21.333z"/>
                    <path style="fill:#cb8c01" d="M42.667 192h320v85.333h-320z"/>
                    <path d="M62.219 234.667h3.562C69.333 245.396 78.521 256 96 256s26.667-10.604 30.219-21.333h3.562C133.333 245.396 142.521 256 160 256s26.667-10.604 30.219-21.333h3.562C197.333 245.396 206.521 256 224 256s26.667-10.604 30.219-21.333h3.562C261.333 245.396 270.521 256 288 256s26.667-10.604 30.219-21.333h3.562C325.333 245.396 334.521 256 352 256a35.058 35.058 0 0 0 10.667-1.678v-30.447c-.125 4.917-2.073 10.792-10.667 10.792s-10.542-5.875-10.667-10.667v-10.667l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792s-10.542-5.875-10.667-10.667v-10.667l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792s-10.542-5.875-10.667-10.667v-10.667l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792s-10.542-5.875-10.667-10.667v-10.667l-42.406.146-.26 10.396c-.125 4.917-2.073 10.792-10.667 10.792S85.458 228.792 85.333 224v-10.667l-42.406.146-.26 10.396v30.447c10.632-3.33 16.816-11.394 19.552-19.655z" style="fill:#b37900"/><path d="M62.219 213.333h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c3.552 10.73 12.74 21.334 30.219 21.334s26.667-10.604 30.219-21.334h3.562c3.552 10.73 12.74 21.334 30.219 21.334a35.058 35.058 0 0 0 10.667-1.679v-30.446c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396c-.125 4.916-2.073 10.791-10.667 10.791s-10.542-5.875-10.667-10.666V192l-42.406.146-.26 10.396v30.446c10.632-3.33 16.816-11.393 19.552-19.655z" style="fill:#d6a436"/><path style="fill:#9e6611" d="M42.667 256h426.667v42.667H42.667z"/><path style="fill:#885400" d="M320 256h149.333v42.667H320z"/><path style="fill:#9e6611" d="M42.667 277.333h64v213.333h-64zM106.667 277.333h64v213.333h-64z"/><path style="fill:#885400" d="M405.333 277.333h64v213.333h-64zM341.333 277.333h64v213.333h-64z"/><path style="fill:#cb8c01" d="M42.667 277.333h42.667v213.333H42.667zM106.667 277.333h42.667v213.333h-42.667zM192 277.333h42.667v213.333H192z"/><path style="fill:#b37900" d="M362.667 277.333h42.667v213.333h-42.667z"/><path style="fill:#cb8c01" d="M277.333 277.333H320v213.333h-42.667z"/><path style="fill:#b37900" d="M426.667 277.333h42.667v213.333h-42.667zM42.667 277.333h42.667V320H42.667zM106.667 277.333h42.667V320h-42.667zM192 277.333h42.667V320H192z"/><path style="fill:#9e6611" d="M362.667 277.333h42.667V320h-42.667z"/><path style="fill:#b37900" d="M277.333 277.333H320V320h-42.667z"/>
                    <path style="fill:#9e6611" d="M426.667 277.333h42.667V320h-42.667z"/>
                    <path style="fill:#e9cf95" d="M149.333 21.333h213.333v21.333H149.333z"/></g>
                    </svg>
                <span>{"Verbihr"}</span>
            </div>
            <div class="flex space-x-4 print:hidden">
            <button onclick={oninfoclick} class="focus:outline-none focus-visible:scale-125 dark:text-black">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6 hover:scale-110 transition-transform duration-300 ease-in-out">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 14v-4M12 6h.01"></path>
                </svg>
            </button>

            <label class="swap swap-rotate dark:text-black hover:scale-110 ">
            <input type="checkbox" class="theme-controller"  ref={theme_ref} />

            <svg
                class="swap-on h-6 w-6 fill-current"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24">
                <path
                d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" />
            </svg>

            <svg
                class="swap-off h-6 w-6 fill-current"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24">
                <path
                d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" />
            </svg>
            </label>
            <select {onchange} ref={select_ref} class="border-0 bg-transparent transition-colors duration-300 focus:outline-none focus-visible:scale-125 dark:text-black">
                {Locale::get_locales().iter().map(|locale|
                    html! { <option value={locale.short_name.to_string()} selected={is_selected(locale)}>{locale.short_name.to_string()}</option> }
                ).collect::<Html>()}
            </select>
            </div>
          </nav>
        }
    }
}

pub mod modal {

    use std::rc::Rc;

    use web_sys::KeyboardEvent;
    use yew::{classes, function_component, html, use_context, Html};

    use crate::{
        components::{use_event_on_context, ONKEYDOWN_EVENT_NAME},
        context::Context,
    };

    const MODAL_ID: &str = "verbihr_modal";

    #[function_component(Modal)]
    pub fn modal() -> Html {
        let context = use_context::<Rc<Context>>().unwrap();

        let is_modal_open = context.is_modal_open.clone();
        let is_modal_open_val = *context.is_modal_open.clone();

        use_event_on_context(
            {
                let is_modal_open = is_modal_open.clone();
                move |keydown: KeyboardEvent| {
                    if keydown.key_code() == 27 {
                        keydown.prevent_default();
                        is_modal_open.set(false);
                    }
                }
            },
            ONKEYDOWN_EVENT_NAME,
        );

        html! {
                    <div id={MODAL_ID} class={classes!["modal", is_modal_open_val.then_some(Some("modal-open"))]}>
                        <div class="modal-box flex flex-col space-y-1">
                            <div class="flex flex-row justify-between">
                                <h3 class="text-lg font-bold">{"Help"}</h3>
                                <button onclick={move |_| is_modal_open.set(false)} class="btn">{"Close"}</button>
                            </div>
                            <table class="table">
                            <thead>
                                <tr>
                                    <th>{"Shortcut"}</th>
                                    <th>{"Output"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td><kbd class="kbd">{"ALT"}</kbd>{"+"}<kbd class="kbd">{"a"}</kbd></td>
                                    <td>{"ä"}</td>
                                </tr>
                                    <tr>
                                    <td><kbd class="kbd">{"ALT"}</kbd>{"+"}<kbd class="kbd">{"i"}</kbd></td>
                                    <td>{"ï"}</td>
                                </tr>
                                <tr>
                                    <td><kbd class="kbd">{"ALT"}</kbd>{"+"}<kbd class="kbd">{"u"}</kbd></td>
                                    <td>{"ü"}</td>
                                </tr>
                                <tr>
                                    <td><kbd class="kbd">{"ALT"}</kbd>{"+"}<kbd class="kbd">{"s"}</kbd></td>
                                    <td>{"ß"}</td>
                                </tr>
                            </tbody>
                            </table>
                        </div>
                </div>
        }
    }
}

pub mod body;
pub mod end;
mod game;

pub mod prelude {
    pub use super::body::Body;
    pub use super::footer::Footer;
    pub use super::header::Header;
}
