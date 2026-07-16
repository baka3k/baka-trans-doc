use crate::{
    document::{
        model::{TranslationUnit, UnitKind},
        xml::{apply_translations, extract_blocks},
    },
    error::{AppError, AppResult},
    storage::package::OoxmlPackage,
};
use regex::Regex;
use std::collections::HashMap;

const BLOCK_PATTERN: &str = r"(?s)<a:p(?:\s[^>]*)?>.*?</a:p>";
const TEXT_PATTERN: &str = r"(?s)<a:t(?:\s[^>]*)?>(.*?)</a:t>";

pub fn validate(package: &OoxmlPackage) -> AppResult<()> {
    let types = package
        .entry("[Content_Types].xml")
        .ok_or_else(|| AppError::InvalidDocument("missing content types".into()))?;
    let content_types = String::from_utf8_lossy(&types.data);
    if content_types.contains("macroEnabled") {
        return Err(AppError::UnsupportedFormat(
            "macro-enabled PowerPoint documents are not supported".into(),
        ));
    }
    if package.entry("ppt/presentation.xml").is_none() {
        return Err(AppError::InvalidDocument(
            "missing ppt/presentation.xml".into(),
        ));
    }
    Ok(())
}

pub fn extract(package: &OoxmlPackage) -> AppResult<(Vec<TranslationUnit>, Vec<String>)> {
    validate(package)?;
    let mut units = Vec::new();
    let mut warnings = Vec::new();
    let mut slides: Vec<_> = package
        .entries
        .iter()
        .filter_map(|entry| slide_number(&entry.name).map(|number| (number, entry)))
        .collect();
    slides.sort_by_key(|(number, _)| *number);
    for (slide, entry) in slides {
        let xml = String::from_utf8(entry.data.clone())
            .map_err(|_| AppError::InvalidDocument(format!("{} is not UTF-8 XML", entry.name)))?;
        units.extend(extract_blocks(
            &xml,
            &entry.name,
            BLOCK_PATTERN,
            TEXT_PATTERN,
            UnitKind::SlideParagraph,
            Some(slide),
            &format!("Slide {slide}, item"),
        )?);
    }
    for entry in package
        .entries
        .iter()
        .filter(|entry| entry.name.starts_with("ppt/diagrams/data") && entry.name.ends_with(".xml"))
    {
        let xml = String::from_utf8(entry.data.clone())
            .map_err(|_| AppError::InvalidDocument(format!("{} is not UTF-8 XML", entry.name)))?;
        let diagram_units = extract_blocks(
            &xml,
            &entry.name,
            BLOCK_PATTERN,
            TEXT_PATTERN,
            UnitKind::DiagramParagraph,
            None,
            "SmartArt item",
        )?;
        if diagram_units.is_empty() && xml.contains("<a:t") {
            warnings.push(format!(
                "SmartArt part {} contains text that could not be mapped safely",
                entry.name
            ));
        }
        units.extend(diagram_units);
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

fn slide_number(name: &str) -> Option<usize> {
    let re = Regex::new(r"^ppt/slides/slide(\d+)\.xml$").expect("constant slide regex is valid");
    re.captures(name)?.get(1)?.as_str().parse().ok()
}
