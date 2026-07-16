use crate::{
    document::DocumentPackage,
    error::{AppError, AppResult},
    job::{
        StartTranslationRequest,
        checkpoint::{self, CHECKPOINT_SCHEMA, CheckpointManifest},
        progress::JobProgress,
        report::JobWarning,
        state::JobStatus,
    },
    storage::{export::export_atomic, package::file_sha256},
    translation::{chunk::plan_atomic_unit, ollama::OllamaClient, retry::translate_validated},
};
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, VecDeque},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter};
use tokio_util::sync::CancellationToken;

pub async fn run(
    app: AppHandle,
    checkpoint_dir: PathBuf,
    request: StartTranslationRequest,
    job_id: String,
    cancellation: CancellationToken,
    resumed: Option<CheckpointManifest>,
) -> AppResult<()> {
    let input = PathBuf::from(&request.input_path);
    let output = output_path(&request)?;
    let config = request.config();
    config.validate().map_err(AppError::Internal)?;
    let input_hash = file_sha256(&input)?;
    let config_hash = format!("{:x}", Sha256::digest(serde_json::to_vec(&config)?));
    let mut manifest = resumed.unwrap_or_else(|| CheckpointManifest {
        schema_version: CHECKPOINT_SCHEMA,
        adapter_version: env!("CARGO_PKG_VERSION").into(),
        job_id: job_id.clone(),
        status: JobStatus::Queued,
        request: request.clone(),
        input_hash: input_hash.clone(),
        config_hash: config_hash.clone(),
        translations: HashMap::new(),
        warnings: Vec::new(),
        updated_at: Utc::now(),
    });
    if !manifest.is_compatible()
        || manifest.input_hash != input_hash
        || manifest.config_hash != config_hash
    {
        return Err(AppError::CheckpointMismatch(
            "input, configuration, schema, or adapter version changed".into(),
        ));
    }
    manifest.status = manifest.status.transition(JobStatus::Running)?;
    manifest.updated_at = Utc::now();
    checkpoint::write(&checkpoint_dir, &manifest)?;

    emit(
        &app,
        JobProgress {
            job_id: job_id.clone(),
            status: JobStatus::Running,
            phase: "extracting".into(),
            current: 0,
            total: 0,
            percent: 0.0,
            eta_seconds: None,
            current_item: None,
            warning: None,
            message: "Reading the Office package safely".into(),
            output_path: None,
        },
    );
    let mut document = DocumentPackage::open(&input)?;
    for (unit_id, translation) in &manifest.translations {
        document.set_translation(unit_id, translation.clone());
    }
    for warning in document.warnings() {
        manifest.warnings.push(JobWarning {
            unit_id: None,
            location: None,
            message: warning.clone(),
        });
    }
    let units = document.units().to_vec();
    let total = units.len();
    let client = OllamaClient::new(&config.endpoint, config.timeout_secs)?;
    let mut durations = VecDeque::new();
    let mut successful = manifest.translations.len();

    for (index, unit) in units.iter().enumerate() {
        if cancellation.is_cancelled() {
            manifest.status = manifest
                .status
                .transition(JobStatus::Cancelling)?
                .transition(JobStatus::Cancelled)?;
            manifest.updated_at = Utc::now();
            checkpoint::write(&checkpoint_dir, &manifest)?;
            emit(
                &app,
                JobProgress {
                    job_id: job_id.clone(),
                    status: JobStatus::Cancelled,
                    phase: "cancelled".into(),
                    current: index,
                    total,
                    percent: percent(index, total),
                    eta_seconds: None,
                    current_item: Some(unit.location.label.clone()),
                    warning: None,
                    message: "Translation cancelled; a recovery checkpoint was kept".into(),
                    output_path: None,
                },
            );
            return Ok(());
        }
        if manifest.translations.contains_key(&unit.id) {
            emit_unit_progress(
                &app,
                &job_id,
                index + 1,
                total,
                unit.location.label.clone(),
                None,
                &durations,
            );
            continue;
        }
        let decision = plan_atomic_unit(&unit.text, config.chunk_chars);
        if decision.exceeds_limit {
            manifest.warnings.push(JobWarning {
                unit_id: Some(unit.id.clone()),
                location: Some(unit.location.clone()),
                message: format!(
                    "Atomic unit exceeds the {} character target and was kept whole",
                    config.chunk_chars
                ),
            });
        }
        let started = Instant::now();
        match translate_validated(&client, &config, &decision.text, &cancellation).await {
            Ok(translated) => {
                if matches!(document_kind_from_path(&input), Some("pptx")) {
                    let source_len = unit.text.chars().count().max(1) as f64;
                    let ratio = translated.chars().count() as f64 / source_len;
                    if ratio > 1.8 {
                        manifest.warnings.push(JobWarning {
                            unit_id: Some(unit.id.clone()),
                            location: Some(unit.location.clone()),
                            message: format!(
                                "Possible slide overflow (text expansion {ratio:.1}×)"
                            ),
                        });
                    }
                }
                document.set_translation(&unit.id, translated.clone());
                manifest.translations.insert(unit.id.clone(), translated);
                successful += 1;
            }
            Err(AppError::Cancelled) => {
                cancellation.cancel();
                continue;
            }
            Err(error @ AppError::ModelUnavailable(_)) => return Err(error),
            Err(error) => manifest.warnings.push(JobWarning {
                unit_id: Some(unit.id.clone()),
                location: Some(unit.location.clone()),
                message: error.to_string(),
            }),
        }
        durations.push_back(started.elapsed());
        if durations.len() > 10 {
            durations.pop_front();
        }
        manifest.updated_at = Utc::now();
        checkpoint::write(&checkpoint_dir, &manifest)?;
        let warning = manifest
            .warnings
            .last()
            .filter(|warning| warning.unit_id.as_deref() == Some(&unit.id))
            .map(|warning| warning.message.clone());
        emit_unit_progress(
            &app,
            &job_id,
            index + 1,
            total,
            unit.location.label.clone(),
            warning,
            &durations,
        );
    }

    if total > 0 && successful == 0 {
        return Err(AppError::OllamaUnavailable(
            "no translation unit completed successfully; no output was written".into(),
        ));
    }
    emit(
        &app,
        JobProgress {
            job_id: job_id.clone(),
            status: JobStatus::Running,
            phase: "exporting".into(),
            current: total,
            total,
            percent: 99.0,
            eta_seconds: None,
            current_item: None,
            warning: None,
            message: "Validating and atomically exporting the translated package".into(),
            output_path: None,
        },
    );
    let package = document.translated_package()?;
    export_atomic(&package, &input, &output)?;
    manifest.status = manifest
        .status
        .transition(if manifest.warnings.is_empty() {
            JobStatus::Completed
        } else {
            JobStatus::CompletedWithWarnings
        })?;
    manifest.updated_at = Utc::now();
    checkpoint::write(&checkpoint_dir, &manifest)?;
    emit(
        &app,
        JobProgress {
            job_id: job_id.clone(),
            status: manifest.status,
            phase: "completed".into(),
            current: total,
            total,
            percent: 100.0,
            eta_seconds: Some(0),
            current_item: None,
            warning: None,
            message: format!(
                "Translation complete with {} warning(s)",
                manifest.warnings.len()
            ),
            output_path: Some(output.display().to_string()),
        },
    );
    checkpoint::discard(&checkpoint_dir, &job_id)?;
    Ok(())
}

pub fn emit_failure(app: &AppHandle, job_id: &str, error: &AppError) {
    emit(
        app,
        JobProgress {
            job_id: job_id.into(),
            status: JobStatus::Failed,
            phase: "failed".into(),
            current: 0,
            total: 0,
            percent: 0.0,
            eta_seconds: None,
            current_item: None,
            warning: Some(error.to_string()),
            message: error.to_string(),
            output_path: None,
        },
    );
}

fn emit(app: &AppHandle, progress: JobProgress) {
    let _ = app.emit("job-progress", progress);
}

fn emit_unit_progress(
    app: &AppHandle,
    job_id: &str,
    current: usize,
    total: usize,
    item: String,
    warning: Option<String>,
    durations: &VecDeque<Duration>,
) {
    let eta_seconds = if durations.len() < 2 {
        None
    } else {
        let average =
            durations.iter().map(Duration::as_secs_f64).sum::<f64>() / durations.len() as f64;
        Some((average * total.saturating_sub(current) as f64).ceil() as u64)
    };
    emit(
        app,
        JobProgress {
            job_id: job_id.into(),
            status: JobStatus::Running,
            phase: "translating".into(),
            current,
            total,
            percent: percent(current, total),
            eta_seconds,
            current_item: Some(item),
            warning,
            message: format!("Translated {current} of {total} units"),
            output_path: None,
        },
    );
}

fn percent(current: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (current as f64 / total as f64 * 98.0).min(98.0)
    }
}

fn output_path(request: &StartTranslationRequest) -> AppResult<PathBuf> {
    let input = Path::new(&request.input_path);
    let stem = input
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| AppError::UnsafeOutput("invalid input file name".into()))?;
    let extension = input
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| AppError::UnsafeOutput("input has no extension".into()))?;
    Ok(Path::new(&request.output_folder).join(format!("{stem}_vi.{extension}")))
}

fn document_kind_from_path(path: &Path) -> Option<&str> {
    path.extension()?.to_str()
}
