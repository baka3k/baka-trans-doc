# Local Document Translator

A desktop application for translating documents locally with Ollama. Document content stays on your computer and the original file is never modified.

Supported files: DOCX, PPTX, XLSX, Markdown, and UTF-8 TXT. PDF and OCR are not supported.

## Requirements

- [Ollama](https://ollama.com/download)
- The `translategemma:4b` model described below

Building from source also requires:

- [Node.js](https://nodejs.org/en/download) `20.19+` or `22.12+`
- [Rust](https://www.rust-lang.org/tools/install) `1.88+`
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) for your operating system

## Set up Ollama

Install and start Ollama. If it is not already running, use:

```bash
ollama serve
```

In another terminal, download Gemma 3 4B and create the model name used by this project:

https://ollama.com/library/translategemma

```bash
ollama pull translategemma:latest
ollama list
```

Ollama must be available at:

```text
http://localhost:11434
```

## Run from source

Open a terminal in the project directory:

```bash
npm ci
npm run tauri dev
```

The Tauri desktop application will open automatically. `npm run dev` is the shorter equivalent command.

## Use the application

1. Confirm that the header shows **Ollama connected**.
2. Select a supported document.
3. Choose the source and target languages.
4. Select `translategemma:4b`.
5. Choose an output folder.
6. Click **Start translation**.

The output file includes the target language code. For example:

```text
document.docx -> document_en.docx
```

Existing output files are not overwritten.

## Build an installer

Install all dependencies, run the quality checks, and create a release build:

```bash
npm ci
npm run check
npm run tauri build
```

`npm run build` is the shorter equivalent of `npm run tauri build`.

Build artifacts are written to:

```text
src-tauri/target/release/bundle/
```

On Windows, the installers are normally created in:

```text
src-tauri/target/release/bundle/msi/*.msi
src-tauri/target/release/bundle/nsis/*-setup.exe
```

The first release build can take several minutes because Rust dependencies must be compiled.

## Troubleshooting

If Ollama is not detected, confirm that it is running and that the model exists:

```bash
ollama list
```

The list must contain `translategemma:4b`. If it does not, repeat the commands in [Set up Ollama](#set-up-ollama), then click **Check** in the application.

For format-specific restrictions, see [Known limitations](docs/known-limitations.md).
