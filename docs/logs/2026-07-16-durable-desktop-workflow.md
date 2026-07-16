# Durable desktop workflow and release gates — 2026-07-16

## Context

Long local-model jobs must remain cancellable, resume validated work after interruption, and provide a responsive Windows-first desktop workflow without presenting partial output as final.

## Change

Tauri commands launch background work and expose start, cancel, recover, resume, and discard operations (`src-tauri/src/commands/mod.rs:74`, `src-tauri/src/commands/mod.rs:103`, `src-tauri/src/commands/mod.rs:148`). The orchestrator hashes input/configuration, validates checkpoint compatibility, checkpoints each unit, emits typed progress/ETA/warnings, and removes the manifest only after successful atomic export (`src-tauri/src/job/orchestrator.rs:24`, `src-tauri/src/job/orchestrator.rs:179`, `src-tauri/src/job/progress.rs:4`). The React UI drives model/file/output selection, cancellation, results, and recovery (`src/app/App.tsx:41`, `src/app/App.tsx:93`, `src/app/App.tsx:141`); its store ignores stale events and prevents percent regression (`src/store/job.ts:9`). Windows CI runs frontend and Rust quality gates (`.github/workflows/ci.yml:7`), and Tauri bundling is enabled with installer metadata (`src-tauri/tauri.conf.json:28`).

## Impact

**Risk level: high.** Cancellation/crashes retain validated units without publishing partial documents, while incompatible inputs cannot resume stale data. Checkpoints contain document text and therefore remain local app data. Office/LibreOffice fixture QA, a second Ollama model, clean-machine install/uninstall, and public signing remain manual release gates.

## Decision

Checkpoint validated translations per unit and keep export terminal; persisting only a percentage cannot support safe resume, and writing partial output risks user confusion. Keep React event-driven and thin while Rust owns policy. A successful bundle build is not public-release readiness until documented compatibility and signing checks pass.

## References

- Plan: [Local Document Translator](../../plans/260716-2039-local-document-translator/plan.md)
- ADR: `docs/decisions/003-job-events-and-checkpoints.md:10`
- `src-tauri/src/job/checkpoint.rs:30`
- `src-tauri/src/job/orchestrator.rs:50`
- `src/app/App.tsx:128`
- `.github/workflows/ci.yml:19`
- `docs/release-checklist.md:10`
