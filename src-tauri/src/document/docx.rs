use crate::{
    document::{
        model::{TranslationUnit, UnitKind},
        xml::{apply_translations, extract_blocks},
    },
    error::{AppError, AppResult},
    storage::package::OoxmlPackage,
};
use std::collections::HashMap;

const BLOCK_PATTERN: &str = r"(?s)<w:p(?:\s[^>]*)?>.*?</w:p>";
const TEXT_PATTERN: &str = r"(?s)<w:t(?:\s[^>]*)?>(.*?)</w:t>";

pub fn validate(package: &OoxmlPackage) -> AppResult<()> {
    let types = package
        .entry("[Content_Types].xml")
        .ok_or_else(|| AppError::InvalidDocument("missing content types".into()))?;
    let content_types = String::from_utf8_lossy(&types.data);
    if content_types.contains("macroEnabled") {
        return Err(AppError::UnsupportedFormat(
            "macro-enabled Word documents are not supported".into(),
        ));
    }
    if package.entry("word/document.xml").is_none() {
        return Err(AppError::InvalidDocument(
            "missing word/document.xml".into(),
        ));
    }
    Ok(())
}

pub fn extract(package: &OoxmlPackage) -> AppResult<(Vec<TranslationUnit>, Vec<String>)> {
    validate(package)?;
    let mut units = Vec::new();
    let mut warnings = Vec::new();
    for entry in &package.entries {
        let (kind, label) = if entry.name == "word/document.xml" {
            (UnitKind::Paragraph, "Paragraph")
        } else if entry.name.starts_with("word/header") && entry.name.ends_with(".xml") {
            (UnitKind::Header, "Header paragraph")
        } else if entry.name.starts_with("word/footer") && entry.name.ends_with(".xml") {
            (UnitKind::Footer, "Footer paragraph")
        } else {
            continue;
        };
        let xml = String::from_utf8(entry.data.clone())
            .map_err(|_| AppError::InvalidDocument(format!("{} is not UTF-8 XML", entry.name)))?;
        if xml.contains("<w:ins") || xml.contains("<w:del") {
            warnings.push(format!(
                "{} contains tracked revisions; affected paragraphs were skipped",
                entry.name
            ));
        }
        units.extend(extract_blocks(
            &xml,
            &entry.name,
            BLOCK_PATTERN,
            TEXT_PATTERN,
            kind,
            None,
            label,
        )?);
    }
    Ok((units, warnings))
}

pub fn apply(
    package: &mut OoxmlPackage,
    units: &[TranslationUnit],
    translations: &HashMap<String, String>,
) -> AppResult<Vec<String>> {
    let mut changed = Vec::new();
    let mut by_part: HashMap<&str, Vec<TranslationUnit>> = HashMap::new();
    for unit in units {
        by_part
            .entry(&unit.location.part)
            .or_default()
            .push(unit.clone());
    }
    for (part, part_units) in by_part {
        if !part_units
            .iter()
            .any(|unit| translations.contains_key(&unit.id))
        {
            continue;
        }
        let entry = package
            .entry_mut(part)
            .ok_or_else(|| AppError::InvalidDocument(format!("missing part {part}")))?;
        let xml = String::from_utf8(entry.data.clone())
            .map_err(|_| AppError::InvalidDocument(format!("{part} is not UTF-8 XML")))?;
        entry.data =
            apply_translations(&xml, TEXT_PATTERN, &part_units, translations)?.into_bytes();
        changed.push(part.to_owned());
    }
    Ok(changed)
}
