pub mod checkpoint;
pub mod orchestrator;
pub mod progress;
pub mod report;
pub mod state;

use crate::config::TranslationConfig;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartTranslationRequest {
    pub input_path: String,
    pub output_folder: String,
    pub endpoint: String,
    pub model: String,
    pub chunk_chars: Option<usize>,
}

impl StartTranslationRequest {
    pub fn config(&self) -> TranslationConfig {
        TranslationConfig {
            endpoint: self.endpoint.clone(),
            model: self.model.clone(),
            chunk_chars: self.chunk_chars.unwrap_or(1_800),
            timeout_secs: 120,
            max_attempts: 3,
        }
    }
}

#[derive(Clone, Default)]
pub struct JobManager {
    jobs: Arc<Mutex<HashMap<String, CancellationToken>>>,
}

impl JobManager {
    pub fn insert(&self, id: String, token: CancellationToken) {
        self.jobs
            .lock()
            .expect("job manager lock poisoned")
            .insert(id, token);
    }
    pub fn cancel(&self, id: &str) -> bool {
        if let Some(token) = self.jobs.lock().expect("job manager lock poisoned").get(id) {
            token.cancel();
            true
        } else {
            false
        }
    }
    pub fn remove(&self, id: &str) {
        self.jobs
            .lock()
            .expect("job manager lock poisoned")
            .remove(id);
    }
}
