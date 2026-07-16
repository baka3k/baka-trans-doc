# English README Setup Guide — 2026-07-16

## Context

The release plan requires installer guidance for Ollama and model setup plus clear troubleshooting (`plans/260716-2039-local-document-translator/phase-07-quality-release.md:12`, `plans/260716-2039-local-document-translator/phase-07-quality-release.md:48`). The existing Vietnamese quick start did not provide a complete English onboarding path for packaged users or source builds.

## Change

Reworked `README.md` into an English product and setup guide. It now distinguishes packaged installation from source builds and documents prerequisites and launch commands (`README.md:31`, `README.md:39`, `README.md:54`). It adds Ollama installation, loopback endpoint constraints, model download recommendations, model/API verification, and end-to-end usage (`README.md:65`, `README.md:85`, `README.md:116`, `README.md:132`). Packaging, common Ollama/build failures, and current format limitations are also documented (`README.md:184`, `README.md:198`, `README.md:236`).

## Impact

Users and contributors now have one English entry point for installing the desktop package, building from source, preparing Ollama, downloading a model, and diagnosing common failures. Risk level: **low**, because the commit changes documentation only; inaccurate model or platform guidance could still cause onboarding confusion as external tooling evolves.

## Decision

Keep the essential end-user and contributor workflow in the root README so a new user can reach a working local translation without first navigating internal documentation. Retain links to the detailed Ollama guide, architecture decisions, release checklist, and known limitations instead of duplicating every implementation detail.

## References

- Plan: `../../plans/260716-2039-local-document-translator/phase-07-quality-release.md:12`
- README: `../../README.md:31`
- README: `../../README.md:65`
- README: `../../README.md:198`
- Commit: `10ee49194ceb52dead75c79875f27cfb1e38da10`
