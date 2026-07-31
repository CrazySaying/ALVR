//! Simple JSON-based localization.
//!
//! The English strings used in the UI are the translation keys: `tr("...")` looks up the key in
//! the currently selected language's map and falls back to the key itself when a translation is
//! missing (or when English is selected). Language files are embedded at compile time through
//! `include_str!`.

use alvr_common::Language;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

static CURRENT_LANGUAGE: RwLock<Language> = RwLock::new(Language::En);
static ZH_CN_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

fn zh_cn_map() -> &'static HashMap<String, String> {
    ZH_CN_MAP.get_or_init(|| {
        serde_json::from_str(include_str!("../resources/lang/zh-CN.json"))
            .expect("failed to parse zh-CN.json")
    })
}

pub fn set_language(language: Language) {
    *CURRENT_LANGUAGE.write().unwrap() = language;
}

pub fn current_language() -> Language {
    *CURRENT_LANGUAGE.read().unwrap()
}

/// Look up `key` in the current language. Falls back to `key` itself (i.e. English) when the
/// current language is English or the key is not translated.
pub fn tr(key: &str) -> String {
    match current_language() {
        Language::En => key.to_owned(),
        Language::ZhCn => zh_cn_map()
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_owned()),
    }
}

/// Translate a format template and substitute `{}` placeholders positionally.
///
/// Unlike `format!`, this works on runtime strings (the translated template may reorder or drop
/// placeholders). Callers pass already-formatted arguments as strings.
pub fn tr_fmt(key: &str, args: &[impl AsRef<str>]) -> String {
    let template = tr(key);

    let mut result = String::new();
    let mut remaining = template.as_str();

    for arg in args {
        match remaining.find("{}") {
            Some(index) => {
                result.push_str(&remaining[..index]);
                result.push_str(arg.as_ref());
                remaining = &remaining[index + 2..];
            }
            None => break,
        }
    }

    result.push_str(remaining);
    result
}
