# Multilingual format expansion — 2026-07-16

## Context

The translator plan called for replacing the fixed Japanese-to-Vietnamese DOCX/PPTX workflow with explicit language pairs and additional structure-preserving formats while retaining offline, immutable-input, atomic-output, and safe-resume guarantees (`plans/260716-2126-multilingual-format-expansion/plan.md:13`).

## Change

The backend now owns a ten-language BCP 47 catalog and rejects unknown or identical language pairs (`src-tauri/src/translation/language.rs:11`, `src-tauri/src/translation/language.rs:74`); prompts, output suffixes, configuration hashes, and schema-2 checkpoints carry that pair (`src-tauri/src/translation/prompt.rs:3`, `src-tauri/src/document/mod.rs:183`, `src-tauri/src/job/checkpoint.rs:13`). `DocumentSession` provides one orchestration contract for DOCX, PPTX, XLSX, Markdown, and TXT (`src-tauri/src/document/mod.rs:26`, `src-tauri/src/document/mod.rs:35`), with shared/inline-string XLSX extraction and source-offset text adapters (`src-tauri/src/document/xlsx.rs:37`, `src-tauri/src/document/markdown.rs:17`, `src-tauri/src/document/text.rs:16`). The UI consumes backend language/capability metadata, while PDF remains explicitly disabled (`src/app/App.tsx:69`, `src-tauri/src/document/mod.rs:233`).

## Impact

**Risk level: high.** Users can translate five enabled document families across catalog-backed language pairs without silently reusing checkpoints from the old contract. The broader parser/export surface raises structure-preservation and text-expansion risk; Office/LibreOffice compatibility and installer checks remain release gates. PDF produces no best-effort output until mapping, Unicode font, searchability, and render-fidelity gates pass (`docs/decisions/004-pdf-translation-strategy.md:18`).

## Decision

Keep language identifiers and capabilities authoritative in Rust, route all enabled formats through `DocumentSession`, and include the language pair in the resume configuration hash. Preserve Markdown/TXT by targeted byte-range replacement and XLSX by minimal OOXML mutation. Retain a typed PDF capability with `canTranslate: false` instead of shipping an unsafe overlay or rebuild implementation.

## References

- Plan: `plans/260716-2126-multilingual-format-expansion/plan.md:13`
- PDF decision: `docs/decisions/004-pdf-translation-strategy.md:18`
- Atomic text export: `src-tauri/src/storage/export.rs:24`
- Commit: `a35f4570020bd3078b5e148a6161fcd7c8448cb8`
