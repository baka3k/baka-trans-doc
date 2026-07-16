# ADR 004: Keep PDF translation capability disabled

## Status

Accepted for the multilingual/format expansion release.

## Context

PDF text is positioned drawing content, not a stable paragraph/run model. A safe adapter must prove reading order and character mapping, remove or replace old glyphs without damaging image/vector/link content, embed target-script Unicode fonts, preserve page geometry, and pass searchable-text plus render regression checks on Windows, macOS, and Linux.

Two approaches were evaluated:

1. A pure-Rust object/content-stream rewrite. This keeps packaging simple, but the available object parsers do not provide a dependable layout-aware text-block replacement layer or universal subset-font/ToUnicode reconstruction.
2. PDFium extraction/render/rebuild. This offers stronger extraction and geometry APIs, but adds a native binary, license notices, platform packaging, version matching, CI assets, and a font-embedding/replacement design. Rebuilding pages also risks losing searchability, accessibility, and link semantics; overlaying text leaves the source text visible/selectable.

## Decision

Neither approach meets the phase acceptance gate in the current release. `DocumentKind::Pdf` and explicit capability metadata exist so clients can explain the limitation, but `canTranslate` is false, the production file picker omits PDF, and starting a PDF job fails before translation or export. Scan-only/OCR, encrypted files, forms/XFA, and outline text remain out of scope.

PDF can be enabled only after fixtures for Latin, Vietnamese, CJK, Korean, Thai, columns, rotation, links, image backgrounds, scan-only, encrypted, and malformed inputs pass all of these gates:

- stable Unicode extraction and reading order with bounding boxes;
- embedded target-script fonts with no system-font dependency;
- unchanged page count, size, and rotation;
- preserved links and non-text graphics;
- searchable translated text with old text removed;
- automated render/text regression plus Windows manual validation;
- documented native license, bundle-size, and cross-platform packaging impact.

## Consequences

DOCX, PPTX, XLSX, Markdown, and TXT ship independently. The UI and docs state that PDF is unavailable rather than claiming unsafe best-effort output. No partial PDF is ever created.
