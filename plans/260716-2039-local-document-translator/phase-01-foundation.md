# Phase 01: Nền tảng và vertical slice

## Bối cảnh

Repo chưa có code. Phase này tạo nền kỹ thuật và chứng minh sớm rằng cách chỉnh OOXML tối thiểu có thể đi xuyên từ UI → Rust → Ollama/mock → file DOCX kết quả.

## Yêu cầu

- Scaffold Tauri 2 với React, TypeScript và Vite.
- Thiết lập format, lint, unit test và CI cơ bản.
- Tách domain core khỏi Tauri command ngay từ đầu.
- Có fixture DOCX tối thiểu và một luồng dịch paragraph end-to-end.

## Kiến trúc

- Tauri commands là adapter mỏng; business logic nằm trong Rust modules.
- Cấu hình serializable, error enum có mã ổn định cho frontend.
- Viết ADR cho OOXML direct mutation, frontend stack và job/event boundary.

## File liên quan

- `src/**`
- `src-tauri/Cargo.toml`
- `src-tauri/src/{lib.rs,main.rs,error.rs,config.rs}`
- `src-tauri/src/{commands,document,translation,job,storage}/**`
- `tests/fixtures/docx/minimal.docx`
- `docs/decisions/**`

## Các bước thực hiện

1. Khởi tạo Tauri app và pin dependency bằng lockfile.
2. Thiết lập `cargo fmt`, `cargo clippy`, Rust tests, TypeScript lint/test và CI Windows.
3. Định nghĩa `AppError`, `DocumentKind`, `TranslationUnit`, `TranslationResult`, `JobProgress` ở mức tối thiểu.
4. Tạo command kiểm tra backend health và typed bridge phía frontend.
5. Làm vertical slice: mở fixture → lấy một `w:t` → gọi fake translator → ghi file tạm → validate ZIP → xuất file mới.
6. So sánh package trước/sau để xác nhận media/relationship không đổi.
7. Ghi ADR và cập nhật README chạy local.

## Todo

- [ ] App mở được trên Windows bằng một lệnh dev.
- [ ] Rust/TypeScript checks chạy tự động.
- [ ] Vertical slice tạo DOCX output mở được.
- [ ] Không overwrite input và không sửa part ngoài text mục tiêu.
- [ ] ADR mô tả ranh giới module và quyết định OOXML.

## Rủi ro

- Chọn XML library không giữ được namespace/whitespace như mong muốn. Dùng vertical slice và diff package để đánh giá trước khi mở rộng.
- Scaffold trộn logic vào command. Review ranh giới module trước Phase 02.

## Tiêu chí thành công

- Có executable desktop, fixture và test end-to-end tối thiểu.
- Quyết định parser/writer được chứng minh bằng diff, không chỉ dựa trên API library.

