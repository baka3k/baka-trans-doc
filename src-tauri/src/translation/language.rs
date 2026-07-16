use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LanguageInfo {
    pub code: &'static str,
    pub native_name: &'static str,
    pub display_name: &'static str,
}

pub const LANGUAGES: &[LanguageInfo] = &[
    LanguageInfo {
        code: "vi",
        native_name: "Tiếng Việt",
        display_name: "Vietnamese",
    },
    LanguageInfo {
        code: "ja",
        native_name: "日本語",
        display_name: "Japanese",
    },
    LanguageInfo {
        code: "en",
        native_name: "English",
        display_name: "English",
    },
    LanguageInfo {
        code: "zh-Hans",
        native_name: "简体中文",
        display_name: "Chinese (Simplified)",
    },
    LanguageInfo {
        code: "zh-Hant",
        native_name: "繁體中文",
        display_name: "Chinese (Traditional)",
    },
    LanguageInfo {
        code: "ko",
        native_name: "한국어",
        display_name: "Korean",
    },
    LanguageInfo {
        code: "th",
        native_name: "ไทย",
        display_name: "Thai",
    },
    LanguageInfo {
        code: "fr",
        native_name: "Français",
        display_name: "French",
    },
    LanguageInfo {
        code: "de",
        native_name: "Deutsch",
        display_name: "German",
    },
    LanguageInfo {
        code: "es",
        native_name: "Español",
        display_name: "Spanish",
    },
];

pub fn catalog() -> &'static [LanguageInfo] {
    LANGUAGES
}

pub fn find(code: &str) -> Option<&'static LanguageInfo> {
    LANGUAGES
        .iter()
        .find(|language| language.code.eq_ignore_ascii_case(code.trim()))
}

pub fn validate_pair(
    source: &str,
    target: &str,
) -> Result<(&'static LanguageInfo, &'static LanguageInfo), String> {
    let source =
        find(source).ok_or_else(|| format!("Unsupported source language code: {source}"))?;
    let target =
        find(target).ok_or_else(|| format!("Unsupported target language code: {target}"))?;
    if source.code == target.code {
        return Err("Source and target languages must be different".into());
    }
    Ok((source, target))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_representative_pairs_and_rejects_invalid_pairs() {
        assert_eq!(validate_pair("ja", "vi").unwrap().1.code, "vi");
        assert_eq!(validate_pair("VI", "en").unwrap().0.code, "vi");
        assert_eq!(validate_pair("zh-hans", "th").unwrap().0.code, "zh-Hans");
        assert!(validate_pair("ja", "ja").is_err());
        assert!(validate_pair("xx", "vi").is_err());
    }
}
