# ADR 005: Backend-owned BCP 47 language catalog

## Status

Accepted.

## Decision

The Rust backend owns the supported language catalog and exposes it through `list_languages`. Requests and checkpoints use canonical BCP 47 codes: `vi`, `ja`, `en`, `zh-Hans`, `zh-Hant`, `ko`, `th`, `fr`, `de`, and `es`.

The backend resolves codes to trusted native/display names before prompt construction. UI-provided names never enter prompts. Start requests are normalized to catalog casing, source and target must differ, and both codes are part of `TranslationConfig`, its hash, output naming, and checkpoint metadata.

Checkpoint schema 2 adds the pair. Schema 1 manifests remain listable and discardable but cannot resume.

## Consequences

Adding a language is a backend contract change. Frontends render catalog data rather than maintaining parallel lists, and filenames use the canonical target code (for example, `report_zh-Hant.docx`).
