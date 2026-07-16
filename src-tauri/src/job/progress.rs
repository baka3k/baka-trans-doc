use crate::job::state::JobStatus;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobProgress {
    pub job_id: String,
    pub status: JobStatus,
    pub phase: String,
    pub current: usize,
    pub total: usize,
    pub percent: f64,
    pub eta_seconds: Option<u64>,
    pub current_item: Option<String>,
    pub warning: Option<String>,
    pub message: String,
    pub output_path: Option<String>,
}
