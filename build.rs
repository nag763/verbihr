use std::{
    collections::HashMap,
    fs::{self, File},
    process::Command,
};

use serde::{Deserialize, Serialize};
use std::io::prelude::*;

const TRANSLATIONS_OUT_PATH: &str = "src/resources/translation.pc";
const IRREGULAR_VERBS_OUT_PATH: &str = "src/resources/irregular_verbs.pc";
const ASSETS_PATH: &str = "assets";
const STYLES_PATH: &str = "assets/tailwind.css";

#[derive(Deserialize)]
struct LocaleDocumentRoot {
    pub locales: Vec<Locale>,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Serialize)]
struct Locale {
    long_name: String,
    short_name: String,
    navigator_names: Vec<String>,
    is_default: bool,
    translations: TranslationMap,
}

#[derive(Deserialize)]
struct GermanVerbRoot {
    pub irregular_verbs: Vec<GermanVerb>,
}

#[derive(Deserialize, Serialize)]
struct GermanVerb {
    infinitiv: String,
    prasens_ich: String,
    prasens_er: String,
    preterit: String,
    partizip_ii: String,
    meaning: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TranslationMap(HashMap<String, String>);

fn main() -> anyhow::Result<(), anyhow::Error> {
    let must_check_assets = {
        if let Ok(profile) = std::env::var("PROFILE") {
            profile != "release"
        } else {
            true
        }
    };
    if must_check_assets {
        if fs::metadata(ASSETS_PATH).is_err() {
            fs::create_dir(ASSETS_PATH)?;
        }
        if fs::metadata(STYLES_PATH).is_err() {
            Command::new("npx")
                .args([
                    "tailwindcss@3.3.0",
                    "-i",
                    "main.css",
                    "-o",
                    "assets/tailwind.css",
                    "-c",
                    "tailwind.config.js",
                ])
                .output()?;
        }
    }
    if fs::metadata(TRANSLATIONS_OUT_PATH).is_err() {
        let translations_yaml: LocaleDocumentRoot =
            serde_yaml::from_str(include_str!("src/resources/translation.yaml"))?;
        let translations_pc = postcard::to_stdvec(&translations_yaml.locales)?;
        let mut trans_file = match File::create(TRANSLATIONS_OUT_PATH) {
            Ok(f) => f,
            Err(_) => File::create(TRANSLATIONS_OUT_PATH)?,
        };
        trans_file.write_all(&translations_pc)?;
    }
    if fs::metadata(IRREGULAR_VERBS_OUT_PATH).is_err() {
        let german_verb_root: GermanVerbRoot =
            serde_yaml::from_str(include_str!("src/resources/irregular_verbs.yaml"))?;
        let german_verbs_pc = postcard::to_stdvec(&german_verb_root.irregular_verbs)?;
        let mut verb_file = match File::create(IRREGULAR_VERBS_OUT_PATH) {
            Ok(f) => f,
            Err(_) => File::create(IRREGULAR_VERBS_OUT_PATH)?,
        };
        verb_file.write_all(&german_verbs_pc)?;
    }
    Ok(())
}
