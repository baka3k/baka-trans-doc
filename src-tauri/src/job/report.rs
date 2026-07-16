use crate::document::model::DocumentLocation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobWarning {
    pub unit_id: Option<String>,
    pub location: Option<DocumentLocation>,
    pub message: String,
}
