use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};
use std::io::prelude::*;

const TRANSLATIONS_IN_PATH : &str = "src/resources/translation.yaml";
const TRANSLATIONS_OUT_PATH : &str = "src/resources/translation.pc";


#[derive(Deserialize)]
struct LocaleDocumentRoot {
    pub locales: Vec<Locale>
}

#[derive(Deserialize, PartialEq, Eq, Clone, Serialize)]
pub struct Locale {
    long_name: String,
    short_name: String,
    navigator_names: Vec<String>,
    is_default: bool,
    pub translations: TranslationMap
}


#[derive(Debug,Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TranslationMap(HashMap<String, String>);

fn main() -> anyhow::Result<(), anyhow::Error> {
    println!("cargo:rerun-if-changed={TRANSLATIONS_IN_PATH}");
    let translations_yaml: LocaleDocumentRoot = serde_yaml::from_str(include_str!("src/resources/translation.yaml"))?;
    let translations_pc = postcard::to_stdvec(&translations_yaml.locales)?;
    let mut file = match File::create(TRANSLATIONS_OUT_PATH) {
        Ok(f) => f,
        Err(_) => File::create(TRANSLATIONS_OUT_PATH)?,
    };
    file.write_all(&translations_pc)?;
    Ok(())
}
