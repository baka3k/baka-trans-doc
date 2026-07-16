# Local Document Translator

Ứng dụng desktop Tauri dịch tài liệu DOCX và PPTX từ tiếng Nhật sang tiếng Việt bằng Ollama cục bộ, ưu tiên giữ nguyên cấu trúc và định dạng OOXML.

## Quick Start

Yêu cầu: Node.js 20+, Rust 1.88+, Tauri prerequisites cho hệ điều hành và Ollama đang chạy tại `http://localhost:11434` với ít nhất một model.

```powershell
npm install
npm run dev
```

## Usage

1. Chọn file `.docx` hoặc `.pptx` không mã hóa và không chứa macro.
2. Chọn model Ollama, thư mục kết quả và bắt đầu dịch.
3. Theo dõi tiến độ theo paragraph hoặc slide. File nguồn không bị sửa; file mới có hậu tố `_vi`.

Ứng dụng chỉ cho phép endpoint Ollama loopback trong MVP. Không có telemetry hoặc cloud API.

## Development

```powershell
npm run lint
npm run test
npm run web:build
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

`npm run check` chạy toàn bộ gate. Xem [kiến trúc](docs/decisions/001-direct-ooxml-mutation.md), [hướng dẫn Ollama](docs/guides/ollama-setup.md) và [giới hạn đã biết](docs/known-limitations.md).

## Packaging

```powershell
npm run build
```

Installer Windows được tạo trong `src-tauri/target/release/bundle`. Public release cần certificate ký code; internal build có thể để unsigned.

