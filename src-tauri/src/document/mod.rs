pub mod docx;
pub mod model;
pub mod pptx;
mod xml;

use crate::{
    document::model::{DocumentKind, InputInspection, TranslationUnit},
    error::{AppError, AppResult},
    storage::package::OoxmlPackage,
};
use std::{collections::HashMap, path::Path};

pub struct DocumentPackage {
    kind: DocumentKind,
    package: OoxmlPackage,
    units: Vec<TranslationUnit>,
    translations: HashMap<String, String>,
    warnings: Vec<String>,
}

impl DocumentPackage {
    pub fn open(path: &Path) -> AppResult<Self> {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let kind = match extension.as_str() {
            "docx" => DocumentKind::Docx,
            "pptx" => DocumentKind::Pptx,
            "docm" | "pptm" => {
                return Err(AppError::UnsupportedFormat(
                    "macro-enabled Office documents are outside the MVP scope".into(),
                ));
            }
            _ => return Err(AppError::UnsupportedFormat(extension)),
        };
        let package = OoxmlPackage::open(path)?;
        let (units, warnings) = match kind {
            DocumentKind::Docx => docx::extract(&package)?,
            DocumentKind::Pptx => pptx::extract(&package)?,
        };
        Ok(Self {
            kind,
            package,
            units,
            translations: HashMap::new(),
            warnings,
        })
    }

    pub fn units(&self) -> &[TranslationUnit] {
        &self.units
    }

    pub fn set_translation(&mut self, unit_id: &str, value: String) {
        self.translations.insert(unit_id.to_owned(), value);
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    pub fn translated_package(mut self) -> AppResult<OoxmlPackage> {
        match self.kind {
            DocumentKind::Docx => {
                docx::apply(&mut self.package, &self.units, &self.translations)?;
            }
            DocumentKind::Pptx => {
                pptx::apply(&mut self.package, &self.units, &self.translations)?;
            }
        }
        Ok(self.package)
    }

    pub fn inspect(path: &Path) -> AppResult<InputInspection> {
        let document = Self::open(path)?;
        let file_name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_owned();
        let stem = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("translated");
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        Ok(InputInspection {
            path: path.display().to_string(),
            file_name,
            kind: document.kind,
            unit_count: document.units.len(),
            character_count: document
                .units
                .iter()
                .map(|unit| unit.text.chars().count())
                .sum(),
            output_name: format!("{stem}_vi.{extension}"),
            warnings: document.warnings,
        })
    }
}

#[cfg(test)]
mod tests;
