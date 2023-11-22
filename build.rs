use std::{
    collections::HashMap,
    fs::{self, File},
    process::Command,
};

use serde::{Deserialize, Serialize};
use std::io::prelude::*;

const TRANSLATIONS_OUT_PATH: &str = "src/resources/translation.pc";
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

#[derive(Deserialize, Serialize)]
struct Verb {
    infinitive: String,
    prasens_ich: String,
    prasens_du: String,
    prasens_er: String,
    prateritum_ich: String,
    partizip_ii: String,
    konjuktiv_ii_ich: String,
    imperativ_singular: String,
    imperativ_plural: String,
    hilfsverb: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TranslationMap(HashMap<String, String>);

fn main() -> anyhow::Result<(), anyhow::Error> {
    let must_check_assets = {
        if let Some(profile) = std::env::var("PROFILE").ok() {
            if profile == "release" {
                false
            } else {
                true
            }
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
    println!("cargo:rerun-if-changed=src/resources/translation.yaml");
    let translations_yaml: LocaleDocumentRoot =
        serde_yaml::from_str(include_str!("src/resources/translation.yaml"))?;
    if fs::metadata(TRANSLATIONS_OUT_PATH).is_err() {
        let translations_pc = postcard::to_stdvec(&translations_yaml.locales)?;
        let mut trans_file = match File::create(TRANSLATIONS_OUT_PATH) {
            Ok(f) => f,
            Err(_) => File::create(TRANSLATIONS_OUT_PATH)?,
        };
        trans_file.write_all(&translations_pc)?;
    }
    Ok(())
}
