use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DocumentKind {
    Docx,
    Pptx,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UnitKind {
    Paragraph,
    Header,
    Footer,
    SlideParagraph,
    DiagramParagraph,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLocation {
    pub label: String,
    pub part: String,
    pub item_index: usize,
    pub slide: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationUnit {
    pub id: String,
    pub kind: UnitKind,
    pub location: DocumentLocation,
    pub text: String,
    pub node_indexes: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputInspection {
    pub path: String,
    pub file_name: String,
    pub kind: DocumentKind,
    pub unit_count: usize,
    pub character_count: usize,
    pub output_name: String,
    pub warnings: Vec<String>,
}
