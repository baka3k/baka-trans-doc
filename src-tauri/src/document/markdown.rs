use crate::{
    document::model::{DocumentLocation, TranslationUnit, UnitKind},
    error::{AppError, AppResult},
};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::{collections::HashMap, fs, ops::Range, path::Path};

const MAX_TEXT_BYTES: u64 = 16 * 1024 * 1024;

pub struct MarkdownDocument {
    source: String,
    bom: bool,
    ranges: HashMap<String, Range<usize>>,
}

impl MarkdownDocument {
    pub fn open(path: &Path) -> AppResult<(Self, Vec<TranslationUnit>, Vec<String>)> {
        let bytes = read_text_file(path)?;
        let bom = bytes.starts_with(&[0xEF, 0xBB, 0xBF]);
        let payload = if bom { &bytes[3..] } else { &bytes };
        let source = String::from_utf8(payload.to_vec())
            .map_err(|_| AppError::InvalidDocument("Markdown files must be valid UTF-8".into()))?;
        let frontmatter_end = frontmatter_end(&source);
        let mut code_depth = 0_usize;
        let mut units = Vec::new();
        let mut ranges = HashMap::new();
        for (event, range) in Parser::new_ext(&source, Options::all()).into_offset_iter() {
            match event {
                Event::Start(Tag::CodeBlock(_)) => code_depth += 1,
                Event::End(TagEnd::CodeBlock) => code_depth = code_depth.saturating_sub(1),
                Event::Text(text)
                    if code_depth == 0
                        && range.start >= frontmatter_end
                        && !text.trim().is_empty() =>
                {
                    let id = format!("markdown:{}", units.len());
                    let label = format!("Markdown text {}", units.len() + 1);
                    units.push(TranslationUnit {
                        id: id.clone(),
                        kind: UnitKind::MarkdownText,
                        location: DocumentLocation {
                            label,
                            part: path.display().to_string(),
                            item_index: units.len() + 1,
                            slide: None,
                        },
                        text: text.into_string(),
                        node_indexes: Vec::new(),
                    });
                    ranges.insert(id, range);
                }
                _ => {}
            }
        }
        Ok((
            Self {
                source,
                bom,
                ranges,
            },
            units,
            Vec::new(),
        ))
    }

    pub fn render(&self, translations: &HashMap<String, String>) -> AppResult<Vec<u8>> {
        let mut replacements: Vec<_> = translations
            .iter()
            .filter_map(|(id, value)| self.ranges.get(id).map(|range| (range.clone(), value)))
            .collect();
        replacements.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
        let mut output = self.source.clone();
        for (range, value) in replacements {
            if range.end > output.len()
                || !output.is_char_boundary(range.start)
                || !output.is_char_boundary(range.end)
            {
                return Err(AppError::InvalidDocument(
                    "Markdown source offsets became invalid".into(),
                ));
            }
            output.replace_range(range, value);
        }
        let mut bytes = Vec::with_capacity(output.len() + usize::from(self.bom) * 3);
        if self.bom {
            bytes.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
        }
        bytes.extend_from_slice(output.as_bytes());
        Ok(bytes)
    }
}

fn read_text_file(path: &Path) -> AppResult<Vec<u8>> {
    let metadata = fs::metadata(path)?;
    if metadata.len() > MAX_TEXT_BYTES {
        return Err(AppError::ResourceLimit(format!(
            "text file exceeds the {} MiB limit",
            MAX_TEXT_BYTES / 1024 / 1024
        )));
    }
    Ok(fs::read(path)?)
}

fn frontmatter_end(source: &str) -> usize {
    if !(source.starts_with("---\n") || source.starts_with("---\r\n")) {
        return 0;
    }
    let mut offset = 0;
    for (index, line) in source.split_inclusive('\n').enumerate() {
        offset += line.len();
        if index > 0 && matches!(line.trim_end_matches(['\r', '\n']), "---" | "...") {
            return offset;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn preserves_markdown_syntax_code_urls_html_and_frontmatter() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("sample.md");
        let source = "---\ntitle: Keep\n---\n# Hello *world*\n\n[Open site](https://example.com) and `code`.\n\n```rs\nlet x = 1;\n```\n<div>raw</div>\n";
        fs::write(&path, source).unwrap();
        let (doc, units, _) = MarkdownDocument::open(&path).unwrap();
        assert!(units.iter().all(|unit| !unit.text.contains("https://")));
        assert!(units.iter().all(|unit| !unit.text.contains("let x")));
        let translations = units
            .iter()
            .map(|unit| (unit.id.clone(), format!("T{}", unit.location.item_index)))
            .collect();
        let output = String::from_utf8(doc.render(&translations).unwrap()).unwrap();
        assert!(output.starts_with("---\ntitle: Keep\n---\n# "));
        assert!(output.contains("*T"));
        assert!(output.contains("](https://example.com)"));
        assert!(output.contains("`code`"));
        assert!(output.contains("```rs\nlet x = 1;\n```"));
        assert!(output.contains("<div>raw</div>"));
    }
}
