use crate::{
    error::{AppError, AppResult},
    job::{StartTranslationRequest, report::JobWarning, state::JobStatus},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub const CHECKPOINT_SCHEMA: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointManifest {
    pub schema_version: u32,
    pub adapter_version: String,
    pub job_id: String,
    pub status: JobStatus,
    pub request: StartTranslationRequest,
    pub input_hash: String,
    pub config_hash: String,
    pub translations: HashMap<String, String>,
    pub warnings: Vec<JobWarning>,
    pub updated_at: DateTime<Utc>,
}

impl CheckpointManifest {
    pub fn is_compatible(&self) -> bool {
        self.schema_version == CHECKPOINT_SCHEMA
            && self.adapter_version == env!("CARGO_PKG_VERSION")
    }
}

pub fn checkpoint_path(directory: &Path, job_id: &str) -> PathBuf {
    directory.join(format!("{job_id}.json"))
}

pub fn write(directory: &Path, manifest: &CheckpointManifest) -> AppResult<()> {
    fs::create_dir_all(directory)?;
    let path = checkpoint_path(directory, &manifest.job_id);
    let temp = directory.join(format!(".{}.tmp", manifest.job_id));
    fs::write(&temp, serde_json::to_vec_pretty(manifest)?)?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    fs::rename(temp, path)?;
    Ok(())
}

pub fn read(directory: &Path, job_id: &str) -> AppResult<CheckpointManifest> {
    let path = checkpoint_path(directory, job_id);
    if !path.exists() {
        return Err(AppError::JobNotFound(job_id.into()));
    }
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

pub fn list(directory: &Path) -> AppResult<Vec<CheckpointManifest>> {
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut manifests = Vec::new();
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        if let Ok(manifest) = serde_json::from_slice::<CheckpointManifest>(&fs::read(path)?)
            && manifest.status.is_recoverable()
        {
            manifests.push(manifest);
        }
    }
    manifests.sort_by_key(|manifest| std::cmp::Reverse(manifest.updated_at));
    Ok(manifests)
}

pub fn discard(directory: &Path, job_id: &str) -> AppResult<()> {
    let path = checkpoint_path(directory, job_id);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn lists_an_old_schema_as_incompatible_with_defaulted_legacy_pair() {
        let directory = tempdir().unwrap();
        let value = serde_json::json!({
            "schemaVersion": 1,
            "adapterVersion": env!("CARGO_PKG_VERSION"),
            "jobId": "legacy",
            "status": "cancelled",
            "request": {
                "inputPath": "old.docx", "outputFolder": ".", "endpoint": "http://localhost:11434",
                "model": "fake", "chunkChars": 1800
            },
            "inputHash": "input", "configHash": "config", "translations": {}, "warnings": [],
            "updatedAt": "2026-07-16T00:00:00Z"
        });
        fs::write(
            directory.path().join("legacy.json"),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
        let listed = list(directory.path()).unwrap();
        assert_eq!(listed.len(), 1);
        assert!(!listed[0].is_compatible());
        assert_eq!(listed[0].request.source_language, "ja");
        assert_eq!(listed[0].request.target_language, "vi");
    }
}
