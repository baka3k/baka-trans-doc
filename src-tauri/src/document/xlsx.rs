use crate::{
    document::{
        model::{TranslationUnit, UnitKind},
        xml::{apply_translations, extract_blocks},
    },
    error::{AppError, AppResult},
    storage::package::OoxmlPackage,
};
use quick_xml::escape::unescape;
use regex::Regex;
use std::collections::HashMap;

const SHARED_BLOCK: &str = r"(?s)<si(?:\s[^>]*)?>.*?</si>";
const INLINE_BLOCK: &str = r"(?s)<is(?:\s[^>]*)?>.*?</is>";
const TEXT_PATTERN: &str = r"(?s)<t(?:\s[^>]*)?>(.*?)</t>";

pub fn validate(package: &OoxmlPackage) -> AppResult<()> {
    let types = package
        .entry("[Content_Types].xml")
        .ok_or_else(|| AppError::InvalidDocument("missing content types".into()))?;
    let content_types = String::from_utf8_lossy(&types.data);
    if content_types.contains("macroEnabled") || content_types.contains("sheet.macroEnabled") {
        return Err(AppError::UnsupportedFormat(
            "macro-enabled Excel workbooks are not supported".into(),
        ));
    }
    if !content_types.contains("spreadsheetml.sheet.main+xml")
        || package.entry("xl/workbook.xml").is_none()
    {
        return Err(AppError::InvalidDocument(
            "missing the XLSX workbook part".into(),
        ));
    }
    Ok(())
}

pub fn extract(package: &OoxmlPackage) -> AppResult<(Vec<TranslationUnit>, Vec<String>)> {
    validate(package)?;
    let sheets = sheet_parts(package)?;
    let mut shared_references: HashMap<usize, Vec<String>> = HashMap::new();
    let mut inline_parts = Vec::new();

    for (part, sheet_name) in &sheets {
        let Some(entry) = package.entry(part) else {
            continue;
        };
        let xml = String::from_utf8(entry.data.clone())
            .map_err(|_| AppError::InvalidDocument(format!("{part} is not UTF-8 XML")))?;
        for (cell, index) in shared_cells(&xml)? {
            shared_references
                .entry(index)
                .or_default()
                .push(format!("{sheet_name}!{cell}"));
        }
        inline_parts.push((part.clone(), sheet_name.clone(), xml));
    }

    let mut units = Vec::new();
    if let Some(entry) = package.entry("xl/sharedStrings.xml") {
        let xml = String::from_utf8(entry.data.clone()).map_err(|_| {
            AppError::InvalidDocument("xl/sharedStrings.xml is not UTF-8 XML".into())
        })?;
        let mut shared_units = extract_blocks(
            &xml,
            &entry.name,
            SHARED_BLOCK,
            TEXT_PATTERN,
            UnitKind::SpreadsheetCell,
            None,
            "Shared string",
        )?;
        for unit in &mut shared_units {
            let index = unit.location.item_index.saturating_sub(1);
            if let Some(references) = shared_references.get(&index) {
                unit.location.label = references
                    .first()
                    .cloned()
                    .unwrap_or_else(|| format!("Shared string {}", index + 1));
            }
        }
        units.extend(shared_units);
    }

    let inline_cell_re = Regex::new(
        r#"(?s)<c\b[^>]*\br="([^"]+)"[^>]*\bt="inlineStr"[^>]*>.*?<is(?:\s[^>]*)?>.*?</is>.*?</c>"#,
    )
    .map_err(|error| AppError::Internal(error.to_string()))?;
    for (part, sheet_name, xml) in inline_parts {
        let cells: Vec<String> = inline_cell_re
            .captures_iter(&xml)
            .filter_map(|capture| capture.get(1).map(|value| value.as_str().to_owned()))
            .collect();
        let mut inline_units = extract_blocks(
            &xml,
            &part,
            INLINE_BLOCK,
            TEXT_PATTERN,
            UnitKind::SpreadsheetCell,
            None,
            "Cell",
        )?;
        for unit in &mut inline_units {
            let index = unit.location.item_index.saturating_sub(1);
            if let Some(cell) = cells.get(index) {
                unit.location.label = format!("{sheet_name}!{cell}");
            }
        }
        units.extend(inline_units);
    }
    Ok((units, vec!["XLSX translation covers shared and inline string cells only; formulas, comments, charts, and sheet names are left unchanged".into()]))
}

pub fn apply(
    package: &mut OoxmlPackage,
    units: &[TranslationUnit],
    translations: &HashMap<String, String>,
) -> AppResult<()> {
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
    }
    Ok(())
}

fn shared_cells(xml: &str) -> AppResult<Vec<(String, usize)>> {
    let cell_re = Regex::new(r#"(?s)<c\b([^>]*)\bt="s"([^>]*)>(.*?)</c>"#)
        .map_err(|error| AppError::Internal(error.to_string()))?;
    let reference_re =
        Regex::new(r#"\br="([^"]+)""#).map_err(|error| AppError::Internal(error.to_string()))?;
    let value_re =
        Regex::new(r"(?s)<v>(\d+)</v>").map_err(|error| AppError::Internal(error.to_string()))?;
    let mut cells = Vec::new();
    for capture in cell_re.captures_iter(xml) {
        let full = capture
            .get(0)
            .map(|value| value.as_str())
            .unwrap_or_default();
        if full.contains("<f") {
            continue;
        }
        let Some(reference) = reference_re.captures(full).and_then(|item| item.get(1)) else {
            continue;
        };
        let Some(index) = value_re
            .captures(full)
            .and_then(|item| item.get(1))
            .and_then(|value| value.as_str().parse().ok())
        else {
            continue;
        };
        cells.push((reference.as_str().to_owned(), index));
    }
    Ok(cells)
}

fn sheet_parts(package: &OoxmlPackage) -> AppResult<Vec<(String, String)>> {
    let workbook = package
        .entry("xl/workbook.xml")
        .ok_or_else(|| AppError::InvalidDocument("missing xl/workbook.xml".into()))?;
    let relationships = package
        .entry("xl/_rels/workbook.xml.rels")
        .ok_or_else(|| AppError::InvalidDocument("missing workbook relationships".into()))?;
    let workbook = String::from_utf8(workbook.data.clone())
        .map_err(|_| AppError::InvalidDocument("workbook XML is not UTF-8".into()))?;
    let relationships = String::from_utf8(relationships.data.clone())
        .map_err(|_| AppError::InvalidDocument("workbook relationships are not UTF-8".into()))?;
    let relationship_re =
        Regex::new(r#"<Relationship\b[^>]*\bId="([^"]+)"[^>]*\bTarget="([^"]+)"[^>]*/?>"#)
            .map_err(|error| AppError::Internal(error.to_string()))?;
    let mut targets = HashMap::new();
    for capture in relationship_re.captures_iter(&relationships) {
        targets.insert(capture[1].to_owned(), resolve_xl_target(&capture[2])?);
    }
    let sheet_re = Regex::new(r#"<sheet\b[^>]*\bname="([^"]+)"[^>]*\br:id="([^"]+)"[^>]*/?>"#)
        .map_err(|error| AppError::Internal(error.to_string()))?;
    let mut sheets = Vec::new();
    for capture in sheet_re.captures_iter(&workbook) {
        let Some(target) = targets.get(&capture[2]) else {
            continue;
        };
        let name = unescape(&capture[1])
            .map_err(|error| AppError::InvalidDocument(error.to_string()))?
            .into_owned();
        sheets.push((target.clone(), name));
    }
    Ok(sheets)
}

fn resolve_xl_target(target: &str) -> AppResult<String> {
    let normalized = target.replace('\\', "/");
    let mut parts = if normalized.starts_with('/') {
        Vec::new()
    } else {
        vec!["xl"]
    };
    for component in normalized.trim_start_matches('/').split('/') {
        match component {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return Err(AppError::InvalidDocument(
                        "unsafe workbook relationship target".into(),
                    ));
                }
            }
            value => parts.push(value),
        }
    }
    let path = parts.join("/");
    if !path.starts_with("xl/") {
        return Err(AppError::InvalidDocument(
            "workbook relationship escapes xl/".into(),
        ));
    }
    Ok(path)
}
