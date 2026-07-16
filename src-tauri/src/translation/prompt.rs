use crate::translation::language::LanguageInfo;

pub fn build_prompt(
    text: &str,
    source: &LanguageInfo,
    target: &LanguageInfo,
    repair: bool,
) -> String {
    let system_prompt = format!(
        r#"You are a professional translator from {} ({}) to {} ({}).

Rules:
- Translate naturally into {}.
- Preserve every paragraph, style marker, and placeholder exactly and in the same order.
- Do not explain or summarize.
- Do not translate URLs, inline code, numbers, placeholders, or formatting markers.
- Return only the translated text."#,
        source.display_name,
        source.native_name,
        target.display_name,
        target.native_name,
        target.display_name,
    );
    let repair_rule = if repair {
        "\nIMPORTANT: A prior response changed protected markers. Copy every ⟦...⟧ token exactly."
    } else {
        ""
    };
    format!("{system_prompt}{repair_rule}\n\nTEXT TO TRANSLATE:\n{text}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translation::language::validate_pair;

    #[test]
    fn builds_a_catalog_backed_language_pair_prompt() {
        let (source, target) = validate_pair("vi", "en").unwrap();
        let prompt = build_prompt("xin chào", source, target, false);
        assert!(prompt.contains("Vietnamese (Tiếng Việt)"));
        assert!(prompt.contains("English (English)"));
    }
}
