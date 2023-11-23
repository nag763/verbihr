use std::rc::Rc;

use yew::{function_component, Properties, html, Html};

use crate::i18n::{TranslationMap, I18N};

#[derive(Properties, PartialEq, Eq)]
pub struct GameProperties {
    #[prop_or_default]
    pub translations: Rc<Option<TranslationMap>>
}

#[function_component(Game)]
pub fn game(props: &GameProperties) -> Html {

    let translations = &props.translations;

    html!{
    <>
        <div class="flex flex-col justify-between h-full">

            <h1 class="text-2xl md:text-4xl font-bold mb-4">{"User Input Table"}</h1>
        
            <table class="border-separate border-spacing-2">
            <thead>
                <tr>
                <th>{"Infinitiv"}</th>
                <th>{"Präsens (ich)"}</th>
                <th>{"Präsens (er)"}</th>
                <th>{"Perfekt"}</th>
                <th>{"Partizip II"}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                <td><input type="text" name="infinitiv" value="" class="table-input"/></td>
                <td><input type="text" name="prasens_ich" value="" class="table-input"/></td>
                <td><input type="text" name="prasens_er" value="" class="table-input"/></td>
                <td><input type="text" name="perfekt" value="" class="table-input"/></td>
                <td><input type="text" name="partizip_ii" value="" class="table-input"/></td>
                </tr>
            </tbody>
            </table>

            <div class="flex items-center justify-center">
            <button class="bg-green-600 text-white py-4 px-8 rounded-lg flex items-center justify-center space-x-2 hover:bg-green-500 transition duration-300 w-2/3 md:w-1/3 h-1/6">
                <span><I18N label={"validate"} {translations}/></span>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                </svg>
            </button>
            </div>
        </div>
    </>
    }
}