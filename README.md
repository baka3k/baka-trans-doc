# Local Document Translator

Local Document Translator is a privacy-first desktop application that translates documents with a locally running [Ollama](https://ollama.com/) model. It is built with Tauri, Rust, React, and TypeScript, and is designed to preserve the structure and formatting of the original document as closely as possible.

All translation requests are sent to an Ollama server on the local machine. The application does not use a cloud translation API, send telemetry, or modify the source file.

## Features

- Local translation through Ollama at `http://localhost:11434`
- DOCX, PPTX, XLSX, Markdown, and UTF-8 TXT support
- Source and target language selection
- Format-aware progress reporting and cancellation
- Recovery checkpoints for interrupted or cancelled jobs
- Atomic export: a completed file is published only after validation
- Source-file protection and no automatic overwrite of existing output files
- No telemetry or cloud API

## Supported formats

| Format | Status | What is translated |
| --- | --- | --- |
| DOCX | Supported | Paragraphs, tables, headers, footers, hyperlinks, lists, and supported text boxes |
| PPTX | Supported | Slide titles, shapes, text boxes, tables, and safely mapped SmartArt text |
| XLSX | Supported | Shared-string and inline-string cells, including rich-text runs |
| Markdown | Supported | Visible text while preserving frontmatter, code, URLs, raw HTML, and source syntax |
| TXT | Supported | UTF-8 text while preserving BOM, line endings, blank lines, and surrounding whitespace |
| PDF | Disabled | PDF and OCR translation are not available in the current release |

Supported languages are Vietnamese, Japanese, English, Simplified Chinese, Traditional Chinese, Korean, Thai, French, German, and Spanish.

## Install for end users

If you received a packaged build, install either the Windows `.msi` package or the NSIS `-setup.exe` package. Node.js and Rust are not required to run a packaged build.

Ollama and at least one downloaded model are still required. Complete the [Ollama setup](#ollama-setup), start Ollama, and then open Local Document Translator.

> Windows packages produced locally are unsigned internal builds unless a code-signing certificate is configured. Windows may display a security warning for an unsigned installer.

## Build from source

### Prerequisites

- [Node.js](https://nodejs.org/en/download) `20.19+` or `22.12+` (Node.js 24 LTS is recommended)
- [Rust](https://www.rust-lang.org/tools/install) `1.88+`, installed through `rustup`
- [Tauri 2 system prerequisites](https://v2.tauri.app/start/prerequisites/) for your operating system
- [Ollama](https://ollama.com/download) with at least one local model

Platform-specific build requirements include:

- **Windows:** Microsoft C++ Build Tools with the **Desktop development with C++** workload and Microsoft Edge WebView2
- **macOS:** Xcode Command Line Tools (`xcode-select --install`)
- **Linux:** WebKitGTK 4.1 and the other packages listed in the Tauri prerequisites for your distribution

### Install dependencies and run

```bash
git clone <repository-url>
cd baka-trans-doc
npm ci
npm run dev
```

`npm run dev` starts the Vite frontend and launches the Tauri desktop application. Running only `npm run web:dev` opens the frontend without native file dialogs or access to Ollama through the Rust backend.

## Ollama setup

### 1. Install Ollama

Download Ollama from the [official download page](https://ollama.com/download). Detailed platform instructions are available for [Windows](https://docs.ollama.com/windows), [macOS](https://docs.ollama.com/macos), and [Linux](https://docs.ollama.com/linux).

On Windows and macOS, the desktop application normally starts the Ollama service in the background. On Linux, or when running Ollama manually, start it with:

```bash
ollama serve
```

The default API address must be available at:

```text
http://localhost:11434
```

For privacy and security, this release accepts only an unauthenticated HTTP endpoint on `localhost`, `127.0.0.1`, or another loopback address. Remote Ollama servers are not supported.

### 2. Download a model

Start with a multilingual instruction model that fits your available memory. `qwen3:4b` is a practical starting point; a larger model can improve translation quality but requires more RAM or VRAM and more disk space.

```bash
# Recommended starting model
ollama pull qwen3:4b

# Larger multilingual option
ollama pull qwen3:8b

# Alternative model
ollama pull gemma3:4b
```

Any local Ollama model may be used, but translation quality and placeholder preservation depend on the model. See the Ollama model pages for [Qwen 3](https://ollama.com/library/qwen3) and [Gemma 3](https://ollama.com/library/gemma3) to compare available variants.

List downloaded models:

```bash
ollama list
```

Optionally test a model before opening the application:

```bash
ollama run qwen3:4b "Translate 'Hello, how are you?' into Vietnamese. Return only the translation."
```

Press `Ctrl+D` or enter `/bye` to leave the interactive Ollama session.

### 3. Verify the Ollama API

On Windows PowerShell:

```powershell
Invoke-RestMethod http://localhost:11434/api/tags
```

On macOS or Linux:

```bash
curl http://localhost:11434/api/tags
```

The response should contain a `models` list. The desktop application reads this endpoint and displays the same models in its model selector.

## Usage

1. Start Ollama and confirm that the status in the application says **Ollama connected**.
2. Click **Browse** under **Source document** and select a supported file.
3. Select different source and target languages.
4. Choose an Ollama model from the detected local models.
5. Keep the default endpoint unless Ollama uses a different loopback port, then click **Check**.
6. Choose an output folder.
7. Click **Start translation** and monitor the progress panel.
8. When the job completes, click **Show output in folder**.

Output files include the target-language code:

```text
report.docx        -> report_en.docx
slides.pptx        -> slides_zh-Hant.pptx
workbook.xlsx      -> workbook_vi.xlsx
```

The source file is never overwritten. If the expected output file already exists, rename or move it, or choose another output folder before retrying.

## Recovery and local data

Incomplete jobs create recovery checkpoints in the operating system's application-data directory. A compatible checkpoint can be resumed or discarded from the recovery banner when the application starts again. Checkpoints can contain already translated text, but remain on the local machine.

Completed jobs remove their checkpoint. Application logs report status and warnings without writing document content to the activity log.

## Development commands

```bash
# Run the desktop app in development mode
npm run dev

# Run the frontend only
npm run web:dev

# Lint TypeScript and React code
npm run lint

# Run frontend tests
npm run test

# Build the frontend
npm run web:build

# Run Rust tests
cargo test --manifest-path src-tauri/Cargo.toml

# Run every automated quality gate
npm run check
```

## Create a desktop package

```bash
npm run build
```

Tauri writes platform-specific installers and bundles to:

```text
src-tauri/target/release/bundle/
```

On Windows, the configured bundle targets produce MSI and NSIS installers. Public distribution should use code signing; an unsigned package is suitable only for internal testing.

## Troubleshooting

### Ollama is not ready or the application cannot connect

```bash
ollama serve
ollama list
```

Confirm that `http://localhost:11434/api/tags` responds. Also check that a firewall or security tool is not blocking loopback traffic. If the Ollama desktop application is already running, do not start a second server on the same port.

### The model selector is empty

Download a model and click **Check** in the application:

```bash
ollama pull qwen3:4b
ollama list
```

The exact name and tag shown by `ollama list` must be available to the application.

### Translation times out or the computer becomes unresponsive

The model may be too large for the available RAM or VRAM, or it may still be loading after a cold start. Try a smaller model such as `qwen3:4b`, close other memory-intensive applications, and retry.

### The translated text does not preserve placeholders

Placeholder and marker behavior varies by model. The application retries invalid responses with stricter instructions, but a unit that still fails is preserved in its original form and reported as a warning. Try another instruction-tuned model if this happens frequently.

### The output file already exists

The application intentionally does not overwrite existing files. Rename, move, or delete the old output only after confirming that it is no longer needed, or choose a different output folder.

### Windows build fails with linker or WebView errors

Revisit the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) and confirm that Microsoft C++ Build Tools and WebView2 are installed. Open a new terminal after installing Rust or build tools so that environment changes are applied.

## Known limitations

- Translation quality depends on the selected Ollama model and language pair.
- DOCX paragraphs containing Track Changes are skipped with a warning. Comments, encrypted files, and `.docm` are unsupported.
- PPTX does not automatically change fonts, geometry, or autofit settings. Speaker notes, animation, and embedded video are outside the current scope.
- XLSX translates text cells only. Formulas, numeric/date values, sheet names, comments, charts, validation, relationships, and media remain unchanged.
- Markdown frontmatter, code, URL destinations, and raw HTML remain unchanged.
- TXT input must be valid UTF-8.
- Macro-enabled Office files (`.docm`, `.pptm`, and `.xlsm`) and legacy `.xls` files are unsupported.
- PDF translation is disabled until text mapping, Unicode font embedding, searchability, and cross-platform rendering meet the release gate. OCR is not included.

See [Known limitations](docs/known-limitations.md) for format-specific details.

## Architecture and project documentation

- [Project specification](docs/specs.md)
- [Ollama setup guide](docs/guides/ollama-setup.md)
- [Direct OOXML mutation decision](docs/decisions/001-direct-ooxml-mutation.md)
- [PDF translation strategy](docs/decisions/004-pdf-translation-strategy.md)
- [Language identifiers and catalog](docs/decisions/005-language-identifiers-and-catalog.md)
- [Release checklist](docs/release-checklist.md)
