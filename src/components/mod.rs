pub mod footer {
    use std::rc::Rc;

    use yew::{function_component, Html, html, use_context};

    use crate::{i18n::I18N, context::Context};

    #[function_component(Footer)]
    pub fn footer() -> Html {
        let context = use_context::<Rc<Context>>().unwrap();
        let translations = &context.translations;
        html! {
            <footer class="text-center sm:py-2 md:py-4 bg-gray-900 h-full text-white text-xs">
            <div class="flex md:flex-col items-center justify-center sm:space-x-2 mdspace-x-4 mt-auto">
                <a href="https://github.com/nag763/verbihr" target="_blank">
                    <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path d="M12 0C5.373 0 0 5.373 0 12c0 5.303 3.438 9.8 8.207 11.387.6.11.793-.258.793-.577 0-.285-.012-1.04-.018-2.04-3.22.702-3.89-1.54-3.89-1.54-.525-1.327-1.282-1.68-1.282-1.68-1.048-.715.08-.702.08-.702 1.16.082 1.773 1.2 1.773 1.2 1.033 1.77 2.713 1.258 3.37.96.105-.748.405-1.26.737-1.546-2.586-.294-5.297-1.293-5.297-5.74 0-1.27.45-2.312 1.2-3.126-.12-.296-.522-1.482.114-3.08 0 0 1.008-.312 3.3 1.2a11.115 11.115 0 012.947-.4c1.002.007 2.007.135 2.947.4 2.29-1.512 3.297-1.2 3.297-1.2.636 1.598.234 2.784.114 3.08.75.814 1.2 1.856 1.2 3.126 0 4.458-2.715 5.442-5.305 5.728.42.36.795 1.068.795 2.15 0 1.55-.015 2.8-.015 3.18 0 .318.21.694.8.576C20.568 21.797 24 16.3 24 12c0-6.627-5.373-12-12-12z"/>
                    </svg>
                </a>
                <p>{"© 2023 LABEYE Loïc. "}<I18N label="all_rights_reserved" {translations} /></p>
                <p class="sm:invisible md:visible"><I18N label="footer" {translations} /> </p>
              </div>
            </footer>
        }
    }
}

pub mod header {

    use yew::{function_component, Html, html};

    #[function_component(Header)]
    pub fn header() -> Html {
        html! {
            <nav class="bg-gradient-to-r from-black via-red-700 to-yellow-500 flex items-center justify-between p-6">
            <div class="text-white font-bold text-xl">{"Verbihr"}</div>
            <div class="space-x-4">
            </div>
          </nav>
        }
    }
}

pub mod body;

pub mod prelude {
    pub use super::header::Header;
    pub use super::footer::Footer;
    pub use super::body::Body;
}