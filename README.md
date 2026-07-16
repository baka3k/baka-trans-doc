# Local Document Translator

Ứng dụng desktop Tauri dịch tài liệu cục bộ bằng Ollama, cho phép chọn ngôn ngữ nguồn/đích và ưu tiên giữ nguyên cấu trúc tài liệu.

Định dạng được bật: DOCX, PPTX, XLSX, Markdown (`.md`, `.markdown`) và TXT UTF-8. PDF có model capability nhưng đang bị tắt vì chưa đạt gate về mapping text, font Unicode và render fidelity; ứng dụng không hỗ trợ OCR.

## Quick Start

Yêu cầu: Node.js 20+, Rust 1.88+, Tauri prerequisites cho hệ điều hành và Ollama đang chạy tại `http://localhost:11434` với ít nhất một model.

```powershell
npm install
npm run dev
```

## Usage

1. Chọn tài liệu được hỗ trợ, ngôn ngữ nguồn/đích và model Ollama.
2. Chọn thư mục kết quả rồi bắt đầu dịch.
3. Theo dõi vị trí do adapter cung cấp. File nguồn không bị sửa; file mới có hậu tố mã ngôn ngữ đích như `_en` hoặc `_zh-Hant`.

Ứng dụng chỉ cho phép endpoint Ollama loopback trong MVP. Không có telemetry hoặc cloud API.

## Development

```powershell
npm run lint
npm run test
npm run web:build
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

`npm run check` chạy toàn bộ gate. Xem [kiến trúc OOXML](docs/decisions/001-direct-ooxml-mutation.md), [quyết định PDF](docs/decisions/004-pdf-translation-strategy.md), [catalog ngôn ngữ](docs/decisions/005-language-identifiers-and-catalog.md), [hướng dẫn Ollama](docs/guides/ollama-setup.md) và [giới hạn đã biết](docs/known-limitations.md).

## Packaging

```powershell
npm run build
```

Installer Windows được tạo trong `src-tauri/target/release/bundle`. Public release cần certificate ký code; internal build có thể để unsigned.
