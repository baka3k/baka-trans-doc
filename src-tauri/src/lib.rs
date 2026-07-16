mod commands;
mod config;
mod document;
mod error;
mod job;
mod storage;
mod translation;

use job::JobManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(JobManager::default())
        .invoke_handler(tauri::generate_handler![
            commands::backend_health,
            commands::inspect_input,
            commands::list_models,
            commands::start_translation,
            commands::cancel_translation,
            commands::list_recoverable_jobs,
            commands::resume_translation,
            commands::discard_job,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Local Document Translator");
}
