use crate::{
    document::{DocumentPackage, model::InputInspection},
    error::{AppError, AppResult},
    job::{
        JobManager, StartTranslationRequest,
        checkpoint::{self, CheckpointManifest},
        orchestrator,
        state::JobStatus,
    },
    translation::ollama::{ModelInfo, OllamaClient},
};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager, State};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendHealth {
    status: &'static str,
    version: &'static str,
    offline_only: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoverableJob {
    job_id: String,
    status: JobStatus,
    input_path: String,
    model: String,
    completed_units: usize,
    warning_count: usize,
    updated_at: String,
}

impl From<CheckpointManifest> for RecoverableJob {
    fn from(value: CheckpointManifest) -> Self {
        Self {
            job_id: value.job_id,
            status: value.status,
            input_path: value.request.input_path,
            model: value.request.model,
            completed_units: value.translations.len(),
            warning_count: value.warnings.len(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

#[tauri::command]
pub fn backend_health() -> BackendHealth {
    BackendHealth {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        offline_only: true,
    }
}

#[tauri::command]
pub async fn inspect_input(path: String) -> AppResult<InputInspection> {
    tauri::async_runtime::spawn_blocking(move || DocumentPackage::inspect(Path::new(&path)))
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?
}

#[tauri::command]
pub async fn list_models(endpoint: String) -> AppResult<Vec<ModelInfo>> {
    OllamaClient::new(&endpoint, 15)?.list_models().await
}

#[tauri::command]
pub fn start_translation(
    app: AppHandle,
    manager: State<'_, JobManager>,
    request: StartTranslationRequest,
) -> AppResult<String> {
    request.config().validate().map_err(AppError::Internal)?;
    let checkpoint_dir = checkpoint_directory(&app)?;
    let job_id = Uuid::new_v4().to_string();
    spawn_job(
        app,
        manager.inner().clone(),
        checkpoint_dir,
        request,
        job_id.clone(),
        None,
    );
    Ok(job_id)
}

#[tauri::command]
pub fn cancel_translation(manager: State<'_, JobManager>, job_id: String) -> AppResult<()> {
    if manager.cancel(&job_id) {
        Ok(())
    } else {
        Err(AppError::JobNotFound(job_id))
    }
}

#[tauri::command]
pub fn list_recoverable_jobs(app: AppHandle) -> AppResult<Vec<RecoverableJob>> {
    Ok(checkpoint::list(&checkpoint_directory(&app)?)?
        .into_iter()
        .map(RecoverableJob::from)
        .collect())
}

#[tauri::command]
pub fn resume_translation(
    app: AppHandle,
    manager: State<'_, JobManager>,
    job_id: String,
) -> AppResult<String> {
    let checkpoint_dir = checkpoint_directory(&app)?;
    let manifest = checkpoint::read(&checkpoint_dir, &job_id)?;
    if !manifest.is_compatible() {
        return Err(AppError::CheckpointMismatch(
            "the checkpoint was created by an incompatible app version".into(),
        ));
    }
    let request = manifest.request.clone();
    spawn_job(
        app,
        manager.inner().clone(),
        checkpoint_dir,
        request,
        job_id.clone(),
        Some(manifest),
    );
    Ok(job_id)
}

#[tauri::command]
pub fn discard_job(app: AppHandle, job_id: String) -> AppResult<()> {
    checkpoint::discard(&checkpoint_directory(&app)?, &job_id)
}

fn checkpoint_directory(app: &AppHandle) -> AppResult<PathBuf> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|error| AppError::Io(error.to_string()))?
        .join("jobs"))
}

fn spawn_job(
    app: AppHandle,
    manager: JobManager,
    checkpoint_dir: PathBuf,
    request: StartTranslationRequest,
    job_id: String,
    resumed: Option<CheckpointManifest>,
) {
    let cancellation = CancellationToken::new();
    manager.insert(job_id.clone(), cancellation.clone());
    tauri::async_runtime::spawn(async move {
        if let Err(error) = orchestrator::run(
            app.clone(),
            checkpoint_dir,
            request,
            job_id.clone(),
            cancellation,
            resumed,
        )
        .await
        {
            orchestrator::emit_failure(&app, &job_id, &error);
        }
        manager.remove(&job_id);
    });
}
