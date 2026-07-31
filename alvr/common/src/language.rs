use serde::{Deserialize, Serialize};
use settings_schema::SettingsSchema;

/// Language used by the dashboard UI. The settings-schema derive generates `LanguageDefault` and
/// `LanguageDefaultVariant` in this module (re-exported through `pub use language::*`).
///
/// Note: do not add `#[serde(rename_all)]` here, as the schema default string, the variant ids and
/// the value stored in session.json all derive from the Rust variant names (`"En"` / `"ZhCn"`).
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    #[schema(strings(display_name = "English"))]
    En,
    #[schema(strings(display_name = "简体中文"))]
    ZhCn,
}

impl Default for Language {
    #[cfg(not(target_arch = "wasm32"))]
    fn default() -> Self {
        sys_locale::get_locale()
            .filter(|locale| locale.to_ascii_lowercase().starts_with("zh"))
            .map_or(Self::En, |_| Self::ZhCn)
    }

    #[cfg(target_arch = "wasm32")]
    fn default() -> Self {
        Self::En
    }
}

impl From<Language> for LanguageDefaultVariant {
    fn from(language: Language) -> Self {
        match language {
            Language::En => Self::En,
            Language::ZhCn => Self::ZhCn,
        }
    }
}

impl From<LanguageDefaultVariant> for Language {
    fn from(variant: LanguageDefaultVariant) -> Self {
        match variant {
            LanguageDefaultVariant::En => Self::En,
            LanguageDefaultVariant::ZhCn => Self::ZhCn,
        }
    }
}
