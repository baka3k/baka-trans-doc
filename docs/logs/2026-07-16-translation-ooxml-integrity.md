# Local translation and OOXML integrity core — 2026-07-16

## Context

The MVP requires Japanese-to-Vietnamese translation through local Ollama while preserving protected content, Office structure, media, relationships, and run formatting. This event covers the Phase 02–04 integrity boundary.

## Change

The backend permits only unauthenticated root HTTP loopback endpoints (`src-tauri/src/config.rs:46`) and uses a deterministic, timed Ollama client (`src-tauri/src/translation/ollama.rs:47`). URLs, code, and numbers are replaced by collision-resistant placeholders; output must retain placeholders and style markers in order, with bounded cancellation-aware retry (`src-tauri/src/translation/protect.rs:26`, `src-tauri/src/translation/retry.rs:9`). DOCX/PPTX adapters map text nodes and write translations back only to their original `w:t`/`a:t` captures (`src-tauri/src/document/docx.rs:32`, `src-tauri/src/document/pptx.rs:33`, `src-tauri/src/document/xml.rs:77`). ZIP loading rejects unsafe paths and resource excess, while export validates a sibling temporary package before atomic rename (`src-tauri/src/storage/package.rs:27`, `src-tauri/src/storage/export.rs:7`). The golden DOCX test verifies bold markup and translated nodes while media and content types remain byte-identical (`src-tauri/src/document/tests.rs:20`).

## Impact

**Risk level: high.** Content stays on the local Ollama boundary, malformed output cannot silently remove protected data or reorder style islands, and unmapped package parts remain unchanged. Real Office/LibreOffice compatibility across uncommon OOXML variants remains a release gate.

## Decision

Use strict model-output validation plus minimal mapped OOXML mutation. Trusting model output or rebuilding via a document object model would violate content/fidelity requirements; Office COM was rejected because it requires Office and breaks the cross-platform architecture. Ambiguous structures generate warnings instead of speculative rewrites.

## References

- Plan: [Local Document Translator](../../plans/260716-2039-local-document-translator/plan.md)
- ADR: `docs/decisions/001-direct-ooxml-mutation.md:10`
- `src-tauri/src/config.rs:33`
- `src-tauri/src/translation/retry.rs:20`
- `src-tauri/src/document/xml.rs:122`
- `src-tauri/src/storage/export.rs:26`

