pub mod docx;
pub mod markdown;
pub mod model;
pub mod pptx;
pub mod text;
pub mod xlsx;
mod xml;

use crate::{
    document::model::{DocumentCapabilities, DocumentKind, InputInspection, TranslationUnit},
    error::{AppError, AppResult},
    storage::{
        export::{export_atomic, export_bytes_atomic},
        package::OoxmlPackage,
    },
    translation::language,
};
use std::{collections::HashMap, fs::File, io::Read, path::Path};

enum Representation {
    Ooxml(OoxmlPackage),
    Markdown(markdown::MarkdownDocument),
    Text(text::TextDocument),
}

pub struct DocumentSession {
    kind: DocumentKind,
    representation: Representation,
    units: Vec<TranslationUnit>,
    translations: HashMap<String, String>,
    warnings: Vec<String>,
}

impl DocumentSession {
    pub fn open(path: &Path) -> AppResult<Self> {
        let kind = kind_from_path(path)?;
        let (representation, units, warnings) = match kind {
            DocumentKind::Docx | DocumentKind::Pptx | DocumentKind::Xlsx => {
                let package = OoxmlPackage::open(path)?;
                let (units, warnings) = match kind {
                    DocumentKind::Docx => docx::extract(&package)?,
                    DocumentKind::Pptx => pptx::extract(&package)?,
                    DocumentKind::Xlsx => xlsx::extract(&package)?,
                    _ => unreachable!(),
                };
                (Representation::Ooxml(package), units, warnings)
            }
            DocumentKind::Markdown => {
                let (document, units, warnings) = markdown::MarkdownDocument::open(path)?;
                (Representation::Markdown(document), units, warnings)
            }
            DocumentKind::Text => {
                let (document, units, warnings) = text::TextDocument::open(path)?;
                (Representation::Text(document), units, warnings)
            }
            DocumentKind::Pdf => {
                return Err(AppError::UnsupportedFormat(
                    "PDF translation is disabled because the layout/font fidelity gate has not passed; OCR is not supported".into(),
                ));
            }
        };
        Ok(Self {
            kind,
            representation,
            units,
            translations: HashMap::new(),
            warnings,
        })
    }

    pub fn units(&self) -> &[TranslationUnit] {
        &self.units
    }
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
    pub fn set_translation(&mut self, unit_id: &str, value: String) {
        self.translations.insert(unit_id.to_owned(), value);
    }

    pub fn expansion_warning(&self, source: &str, translated: &str) -> Option<String> {
        let threshold = match self.kind {
            DocumentKind::Pptx => 1.8,
            DocumentKind::Xlsx => 2.0,
            _ => return None,
        };
        let ratio = translated.chars().count() as f64 / source.chars().count().max(1) as f64;
        (ratio > threshold).then(|| {
            format!(
                "Possible {} overflow (text expansion {ratio:.1}×)",
                match self.kind {
                    DocumentKind::Pptx => "slide",
                    DocumentKind::Xlsx => "cell",
                    _ => "layout",
                }
            )
        })
    }

    pub fn export_atomic(mut self, input: &Path, output: &Path) -> AppResult<()> {
        match &mut self.representation {
            Representation::Ooxml(package) => {
                match self.kind {
                    DocumentKind::Docx => {
                        docx::apply(package, &self.units, &self.translations)?;
                    }
                    DocumentKind::Pptx => {
                        pptx::apply(package, &self.units, &self.translations)?;
                    }
                    DocumentKind::Xlsx => xlsx::apply(package, &self.units, &self.translations)?,
                    _ => unreachable!(),
                }
                export_atomic(package, input, output)
            }
            Representation::Markdown(document) => {
                let bytes = document.render(&self.translations)?;
                export_bytes_atomic(&bytes, input, output)
            }
            Representation::Text(document) => {
                let bytes = document.render(&self.translations);
                export_bytes_atomic(&bytes, input, output)
            }
        }
    }

    #[cfg(test)]
    fn translated_package(self) -> AppResult<OoxmlPackage> {
        let Representation::Ooxml(mut package) = self.representation else {
            return Err(AppError::Internal("not an OOXML document".into()));
        };
        match self.kind {
            DocumentKind::Docx => {
                docx::apply(&mut package, &self.units, &self.translations)?;
            }
            DocumentKind::Pptx => {
                pptx::apply(&mut package, &self.units, &self.translations)?;
            }
            DocumentKind::Xlsx => xlsx::apply(&mut package, &self.units, &self.translations)?,
            _ => unreachable!(),
        }
        Ok(package)
    }

    pub fn inspect(path: &Path, target_language: &str) -> AppResult<InputInspection> {
        let target = language::find(target_language).ok_or_else(|| {
            AppError::Internal(format!(
                "Unsupported target language code: {target_language}"
            ))
        })?;
        let kind = kind_from_path(path)?;
        if kind == DocumentKind::Pdf {
            let mut header = [0_u8; 5];
            File::open(path)?.read_exact(&mut header)?;
            if &header != b"%PDF-" {
                return Err(AppError::InvalidDocument("invalid PDF header".into()));
            }
            return Ok(InputInspection {
                path: path.display().to_string(), file_name: file_name(path), kind,
                unit_count: 0, character_count: 0,
                output_name: output_file_name(path, target.code)?,
                warnings: vec!["PDF translation is unavailable in this build; text mapping, Unicode font embedding, and render fidelity have not passed the release gate".into()],
                capabilities: capabilities(kind),
            });
        }
        let document = Self::open(path)?;
        Ok(InputInspection {
            path: path.display().to_string(),
            file_name: file_name(path),
            kind: document.kind,
            unit_count: document.units.len(),
            character_count: document
                .units
                .iter()
                .map(|unit| unit.text.chars().count())
                .sum(),
            output_name: output_file_name(path, target.code)?,
            warnings: document.warnings,
            capabilities: capabilities(document.kind),
        })
    }
}

pub fn output_file_name(path: &Path, target_language: &str) -> AppResult<String> {
    let target = language::find(target_language)
        .ok_or_else(|| AppError::UnsafeOutput("unsupported target language".into()))?;
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| AppError::UnsafeOutput("invalid input file name".into()))?;
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| AppError::UnsafeOutput("input has no extension".into()))?;
    Ok(format!("{stem}_{}.{extension}", target.code))
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_owned()
}

fn kind_from_path(path: &Path) -> AppResult<DocumentKind> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match extension.as_str() {
        "docx" => Ok(DocumentKind::Docx),
        "pptx" => Ok(DocumentKind::Pptx),
        "xlsx" => Ok(DocumentKind::Xlsx),
        "pdf" => Ok(DocumentKind::Pdf),
        "md" | "markdown" => Ok(DocumentKind::Markdown),
        "txt" => Ok(DocumentKind::Text),
        "docm" | "pptm" | "xlsm" => Err(AppError::UnsupportedFormat(
            "macro-enabled Office documents are not supported".into(),
        )),
        "xls" => Err(AppError::UnsupportedFormat(
            "legacy binary .xls workbooks are not supported".into(),
        )),
        _ => Err(AppError::UnsupportedFormat(extension)),
    }
}

fn capabilities(kind: DocumentKind) -> DocumentCapabilities {
    match kind {
        DocumentKind::Docx | DocumentKind::Pptx => DocumentCapabilities { can_translate: true, limitations: Vec::new() },
        DocumentKind::Xlsx => DocumentCapabilities { can_translate: true, limitations: vec!["Only shared and inline string cells are translated; formulas, sheet names, comments, charts, and metadata are preserved unchanged".into()] },
        DocumentKind::Markdown => DocumentCapabilities { can_translate: true, limitations: vec!["Frontmatter, code, URLs, and raw HTML are preserved unchanged".into()] },
        DocumentKind::Text => DocumentCapabilities { can_translate: true, limitations: vec!["TXT input must be UTF-8; BOM, line endings, blank lines, and surrounding whitespace are preserved".into()] },
        DocumentKind::Pdf => DocumentCapabilities { can_translate: false, limitations: vec!["PDF is disabled until searchable-text mapping, embedded Unicode fonts, and render fidelity pass the release gate; OCR is not included".into()] },
    }
}

#[cfg(test)]
mod tests;
