use std::{collections::HashMap, sync::OnceLock};

use serde::{Deserialize, Serialize};

static VERBS: OnceLock<Vec<GermanVerb>> = OnceLock::new();

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct GermanVerb {
    pub infinitiv: String,
    pub prasens_ich: String,
    pub prasens_er: String,
    pub preterit: String,
    pub partizip_ii: String,
    pub meaning: HashMap<String, String>,
}

impl GermanVerb {
    fn init() -> Vec<GermanVerb> {
        postcard::from_bytes(include_bytes!("resources/irregular_verbs.pc")).unwrap()
    }

    pub fn get_verbs() -> Vec<GermanVerb> {
        VERBS.get_or_init(Self::init).clone()
    }
}
