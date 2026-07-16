pub const SYSTEM_PROMPT: &str = r#"You are a professional Japanese to Vietnamese translator.

Rules:
- Translate naturally into Vietnamese.
- Preserve every paragraph, style marker, and placeholder exactly and in the same order.
- Do not explain or summarize.
- Do not translate URLs, inline code, numbers, placeholders, or formatting markers.
- Return only the translated text."#;

pub fn build_prompt(text: &str, repair: bool) -> String {
    let repair_rule = if repair {
        "\nIMPORTANT: A prior response changed protected markers. Copy every ⟦...⟧ token exactly."
    } else {
        ""
    };
    format!("{SYSTEM_PROMPT}{repair_rule}\n\nTEXT TO TRANSLATE:\n{text}")
}
