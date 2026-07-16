use crate::{
    document::model::{DocumentLocation, TranslationUnit, UnitKind},
    error::{AppError, AppResult},
};
use std::{collections::HashMap, fs, ops::Range, path::Path};

const MAX_TEXT_BYTES: u64 = 16 * 1024 * 1024;

pub struct TextDocument {
    source: String,
    bom: bool,
    ranges: HashMap<String, Range<usize>>,
}

impl TextDocument {
    pub fn open(path: &Path) -> AppResult<(Self, Vec<TranslationUnit>, Vec<String>)> {
        let metadata = fs::metadata(path)?;
        if metadata.len() > MAX_TEXT_BYTES {
            return Err(AppError::ResourceLimit(format!(
                "text file exceeds the {} MiB limit",
                MAX_TEXT_BYTES / 1024 / 1024
            )));
        }
        let bytes = fs::read(path)?;
        let bom = bytes.starts_with(&[0xEF, 0xBB, 0xBF]);
        let payload = if bom { &bytes[3..] } else { &bytes };
        let source = String::from_utf8(payload.to_vec())
            .map_err(|_| AppError::InvalidDocument("TXT files must be valid UTF-8".into()))?;
        let mut units = Vec::new();
        let mut ranges = HashMap::new();
        let mut offset = 0;
        for line in source.split_inclusive('\n') {
            let content = line.trim_end_matches(['\r', '\n']);
            let trimmed_start = content.trim_start();
            let trimmed = trimmed_start.trim_end();
            if !trimmed.is_empty() {
                let start = offset + content.len() - trimmed_start.len();
                let end = start + trimmed.len();
                let id = format!("text:{}", units.len());
                units.push(TranslationUnit {
                    id: id.clone(),
                    kind: UnitKind::TextLine,
                    location: DocumentLocation {
                        label: format!("Text line {}", units.len() + 1),
                        part: path.display().to_string(),
                        item_index: units.len() + 1,
                        slide: None,
                    },
                    text: trimmed.to_owned(),
                    node_indexes: Vec::new(),
                });
                ranges.insert(id, start..end);
            }
            offset += line.len();
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

    pub fn render(&self, translations: &HashMap<String, String>) -> Vec<u8> {
        let mut replacements: Vec<_> = translations
            .iter()
            .filter_map(|(id, value)| self.ranges.get(id).map(|range| (range.clone(), value)))
            .collect();
        replacements.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
        let mut output = self.source.clone();
        for (range, value) in replacements {
            output.replace_range(range, value);
        }
        let mut bytes = Vec::with_capacity(output.len() + usize::from(self.bom) * 3);
        if self.bom {
            bytes.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
        }
        bytes.extend_from_slice(output.as_bytes());
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn preserves_bom_crlf_blank_lines_and_whitespace() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("sample.txt");
        let mut source = vec![0xEF, 0xBB, 0xBF];
        source.extend_from_slice(b"  first  \r\n\r\nsecond\r\n");
        fs::write(&path, source).unwrap();
        let (doc, units, _) = TextDocument::open(&path).unwrap();
        let translations = HashMap::from([
            (units[0].id.clone(), "one".into()),
            (units[1].id.clone(), "two".into()),
        ]);
        assert_eq!(
            doc.render(&translations),
            b"\xEF\xBB\xBF  one  \r\n\r\ntwo\r\n"
        );
    }
}
