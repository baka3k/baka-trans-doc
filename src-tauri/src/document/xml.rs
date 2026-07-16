use crate::{
    document::model::{DocumentLocation, TranslationUnit, UnitKind},
    error::{AppError, AppResult},
};
use quick_xml::escape::{escape, unescape};
use regex::Regex;
use std::collections::HashMap;

pub fn extract_blocks(
    xml: &str,
    part: &str,
    block_pattern: &str,
    text_pattern: &str,
    kind: UnitKind,
    slide: Option<usize>,
    label_prefix: &str,
) -> AppResult<Vec<TranslationUnit>> {
    let block_re =
        Regex::new(block_pattern).map_err(|error| AppError::Internal(error.to_string()))?;
    let text_re =
        Regex::new(text_pattern).map_err(|error| AppError::Internal(error.to_string()))?;
    let all_nodes: Vec<_> = text_re.captures_iter(xml).collect();
    let mut units = Vec::new();

    for (block_index, block) in block_re.find_iter(xml).enumerate() {
        let block_text = block.as_str();
        if matches!(
            kind,
            UnitKind::Paragraph | UnitKind::Header | UnitKind::Footer
        ) && (block_text.contains("<w:ins") || block_text.contains("<w:del"))
        {
            continue;
        }

        let mut node_indexes = Vec::new();
        let mut fragments = Vec::new();
        for (node_index, captures) in all_nodes.iter().enumerate() {
            let full = captures
                .get(0)
                .expect("the full regex capture always exists");
            if full.start() < block.start() || full.end() > block.end() {
                continue;
            }
            let encoded = captures
                .get(1)
                .map(|value| value.as_str())
                .unwrap_or_default();
            let decoded = unescape(encoded)
                .map_err(|error| AppError::InvalidDocument(error.to_string()))?
                .into_owned();
            if decoded.is_empty() {
                continue;
            }
            node_indexes.push(node_index);
            fragments.push(decoded);
        }
        if fragments.iter().all(|fragment| fragment.trim().is_empty()) {
            continue;
        }
        let text = marked_text(&fragments);
        units.push(TranslationUnit {
            id: format!("{part}:{block_index}"),
            kind: kind.clone(),
            location: DocumentLocation {
                label: format!("{label_prefix} {}", block_index + 1),
                part: part.into(),
                item_index: block_index + 1,
                slide,
            },
            text,
            node_indexes,
        });
    }
    Ok(units)
}

pub fn apply_translations(
    xml: &str,
    text_pattern: &str,
    units: &[TranslationUnit],
    translations: &HashMap<String, String>,
) -> AppResult<String> {
    let text_re =
        Regex::new(text_pattern).map_err(|error| AppError::Internal(error.to_string()))?;
    let nodes: Vec<_> = text_re.captures_iter(xml).collect();
    let mut replacements: HashMap<usize, String> = HashMap::new();

    for unit in units {
        let Some(translated) = translations.get(&unit.id) else {
            continue;
        };
        let segments = split_marked_text(translated, unit.node_indexes.len())?;
        for (node_index, segment) in unit.node_indexes.iter().zip(segments) {
            replacements.insert(*node_index, escape(&segment).into_owned());
        }
    }

    let mut output = xml.to_owned();
    for (node_index, captures) in nodes.into_iter().enumerate().rev() {
        let Some(replacement) = replacements.get(&node_index) else {
            continue;
        };
        let content = captures
            .get(1)
            .ok_or_else(|| AppError::InvalidDocument("text node capture is missing".into()))?;
        output.replace_range(content.start()..content.end(), replacement);
    }
    Ok(output)
}

pub(crate) fn marked_text(fragments: &[String]) -> String {
    if fragments.len() == 1 {
        return fragments[0].clone();
    }
    fragments
        .iter()
        .enumerate()
        .map(|(index, fragment)| format!("⟦S{index}⟧{fragment}"))
        .collect()
}

fn split_marked_text(value: &str, expected: usize) -> AppResult<Vec<String>> {
    if expected == 1 {
        return Ok(vec![value.to_owned()]);
    }
    let marker_re = Regex::new(r"⟦S(\d+)⟧").expect("constant marker regex is valid");
    let markers: Vec<_> = marker_re.find_iter(value).collect();
    if markers.len() != expected {
        return Err(AppError::InvalidTranslation(format!(
            "expected {expected} formatting markers, received {}",
            markers.len()
        )));
    }
    let mut result = Vec::with_capacity(expected);
    for (index, marker) in markers.iter().enumerate() {
        if marker.as_str() != format!("⟦S{index}⟧") {
            return Err(AppError::InvalidTranslation(
                "formatting markers changed order".into(),
            ));
        }
        let end = markers
            .get(index + 1)
            .map_or(value.len(), |next| next.start());
        result.push(value[marker.end()..end].to_owned());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_multi_run_translation_without_changing_tags() {
        let xml =
            r#"<w:p><w:r><w:t>赤</w:t></w:r><w:r><w:rPr><w:b/></w:rPr><w:t>青</w:t></w:r></w:p>"#;
        let units = extract_blocks(
            xml,
            "word/document.xml",
            r"(?s)<w:p(?:\s[^>]*)?>.*?</w:p>",
            r"(?s)<w:t(?:\s[^>]*)?>(.*?)</w:t>",
            UnitKind::Paragraph,
            None,
            "Paragraph",
        )
        .unwrap();
        let mut translations = HashMap::new();
        translations.insert(units[0].id.clone(), "⟦S0⟧đỏ⟦S1⟧xanh".into());
        let output = apply_translations(
            xml,
            r"(?s)<w:t(?:\s[^>]*)?>(.*?)</w:t>",
            &units,
            &translations,
        )
        .unwrap();
        assert!(output.contains("<w:t>đỏ</w:t>"));
        assert!(output.contains("<w:b/>"));
        assert!(output.contains("<w:t>xanh</w:t>"));
    }
}
