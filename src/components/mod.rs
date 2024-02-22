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
            <footer class="text-center py-1 lg:py-2 bg-slate-100 dark:bg-gray-800 h-full text-black dark:text-white text-xs h-full print:hidden ">
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

    use web_sys::HtmlSelectElement;
    use yew::{function_component, html, use_context, Html, NodeRef};

    use crate::{context::Context, i18n::Locale};

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

        let user_locale = &context.locale;
        let is_modal_open = &context.is_modal_open;
        let select_ref = NodeRef::default();
        let dark_mode = context.dark_mode.clone();
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

        let onclick = create_callback_with_local_clone!(dark_mode.set(!(*dark_mode)), dark_mode);

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
            <button onclick={oninfoclick} class="focus:outline-none focus-visible:scale-125">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6 hover:scale-110 transition-transform duration-300 ease-in-out">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 14v-4M12 6h.01"></path>
                </svg>
            </button>

            <button {onclick} class="w-6 h-6 text-black transform transition-transform duration-300 hover:rotate-180 focus:outline-none focus-visible:scale-125">
                if *context.dark_mode {
                    {dark_mode_icon}
                } else {
                    {light_mode_icon}
                }
            </button>
            <select {onchange} ref={select_ref} class="border-0 bg-transparent hover:text-blue-500 transition-colors duration-300 focus:outline-none focus-visible:scale-125">
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

    use web_sys::{KeyboardEvent, MouseEvent, EventTarget, Element};
    use yew::{function_component, html, use_context, Html};
    use wasm_bindgen::JsCast;

    use crate::{
        components::{use_event_on_context, ONKEYDOWN_EVENT_NAME},
        context::Context,
        i18n::I18N,
    };

    const MODAL_ID : &str = "verbihr_modal";

    #[function_component(Modal)]
    pub fn modal() -> Html {
        let context = use_context::<Rc<Context>>().unwrap();

        let translations = &context.translations;

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

        let onoutsideclick = {
                let is_modal_open = is_modal_open.clone();
                move |me: MouseEvent| {
                    let target: Option<EventTarget> = me.target();
                    let element = target.and_then(|t| t.dyn_into::<Element>().ok());
                    if let Some(element) = element {
                        if let Ok(maybe_undefined_element) =
                            element.closest(&format!("#{MODAL_ID}"))
                        {
                            if maybe_undefined_element.is_none() {
                                is_modal_open.set(false);
                            }
                        }
                    }
                }
        };


        html! {
            <>
            if is_modal_open_val {
                <div onclick={onoutsideclick} class="absolute left-0 top-0 bg-black bg-opacity-60 h-full w-full z-100">
                    <div class="grid grid-cols-9 items-center justify-center text-black dark:text-white p-6 md:p-12 h-full">
                        <div class="col-span-1 sm:col-span-2 lg:col-span-3"></div>
                        <div class="col-span-7 sm:col-span-5 lg:col-span-3 bg-slate-300 dark:bg-gray-800 border-2 rounded-lg border-slate-400 dark:border-gray-700">
                            <div id={MODAL_ID} name="modal-content flex flex-col space-y-4 px-2" >
                                <div class="flex justify-between border-b border-slate-400 dark:border-gray-700">
                                    <h1 class="pl-2">
                                    <I18N label="help_modal_title" {translations} />
                                    </h1>
                                    <svg onclick={move |_| is_modal_open.set(false)}xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                    </svg>
                                </div>
                                <div class="flex flex-col space-y-2 mb-2">
                                    <h2 class="p-2"><I18N label="keybindings" {translations} /></h2>
                                    <div class="grid grid-cols-4 items-center justify-center text-center gap-4 w-full">
                                        <div class="flex space-x-0.5 items-center justify-center ">
                                            <kbd class="common-kbd">{"ALT"}</kbd><p>{"+"}</p><kbd class="common-kbd">{"a"}</kbd>
                                        </div>
                                        <div>
                                            {"ä"}
                                        </div>
                                        <div class="flex space-x-0.5 items-center justify-center ">
                                            <kbd class="common-kbd">{"ALT"}</kbd><p>{"+"}</p><kbd class="common-kbd">{"i"}</kbd>
                                        </div>
                                        <div>
                                            {"ï"}
                                        </div>
                                        <div class="flex space-x-0.5 items-center justify-center ">
                                            <kbd class="common-kbd">{"ALT"}</kbd><p>{"+"}</p><kbd class="common-kbd">{"u"}</kbd>
                                        </div>
                                        <div>
                                            {"ü"}
                                        </div>
                                        <div class="flex space-x-0.5 items-center justify-center ">
                                            <kbd class="common-kbd">{"ALT"}</kbd><p>{"+"}</p><kbd class="common-kbd">{"s"}</kbd>
                                        </div>
                                        <div>
                                            {"ß"}
                                        </div>
                                    </div>
                                </div>

                            </div>
                        </div>
                        <div class="col-span-1 sm:col-span-2 lg:col-span-3"></div>
                    </div>
                </div>
            }
            </>
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
