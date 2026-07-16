# Local Document Translator MVP — 2026-07-16

## Context

The repository contained only the product specification and the seven-phase implementation plan. The session implemented the offline DOCX/PPTX translation MVP described by that plan.

## Change

- Added bounded OOXML package reading, mapped DOCX/PPTX text extraction, minimal XML text replacement, and atomic safe export (`src-tauri/src/document/xml.rs:9`, `src-tauri/src/storage/export.rs:7`).
- Added marker/placeholder validation, a localhost-only Ollama client, retry/cancellation, checkpoints, resume, typed progress, and partial-failure reporting (`src-tauri/src/config.rs:46`, `src-tauri/src/job/orchestrator.rs:24`).
- Added the typed React desktop workflow and stale-event protection (`src/app/App.tsx:30`, `src/store/job.ts:9`).

## Impact

Users can translate supported Japanese DOCX/PPTX text through a local Ollama model without overwriting the input. Risk level: **high**, because OOXML fidelity depends on real Office compatibility coverage beyond the synthetic fixtures.

## Decision

The implementation mutates only mapped `w:t`/`a:t` content in the source package instead of rebuilding documents. This preserves unsupported relationships, media, styles, and geometry more reliably while keeping the domain core independent of Tauri.

## References

- plan: `../../plans/260716-2039-local-document-translator/plan.md`
- ADR: `../decisions/001-direct-ooxml-mutation.md`
- ADR: `../decisions/003-job-events-and-checkpoints.md`

