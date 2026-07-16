# Phase 04: Hỗ trợ PDF có text

## Bối cảnh

PDF không có mô hình paragraph/run ổn định như OOXML. Text có thể dùng font subset, encoding tùy chỉnh, transform matrix hoặc thứ tự content stream khác thứ tự đọc. Đây là phase có rủi ro kỹ thuật và đóng gói cao nhất.

## Yêu cầu

- Chỉ hỗ trợ PDF có text, không mã hóa và có mapping ký tự/position đủ tin cậy.
- Từ chối scan-only/DRM/password/XFA với lỗi hướng dẫn rõ ràng; không âm thầm tạo output trống.
- Giữ page count, page size, rotation, image/vector content và link annotations khi chiến lược chọn cho phép.
- Nhúng font Unicode hợp lệ cho language đích và cảnh báo overflow/clipping.
- Có render regression và text extraction verification trước khi bật trong production file filter.

## Kiến trúc

Thực hiện spike trước code production và ghi ADR so sánh ít nhất hai hướng: sửa content stream/vector text bằng thư viện PDF thuần Rust, hoặc dùng PDFium/native engine để extract/render/rebuild. Tiêu chí quyết định gồm fidelity, mapping position, Unicode font embedding, license, binary size, cross-platform packaging và khả năng test headless.

Adapter production phải xuất `TranslationUnit` theo text block/page, giữ bounding boxes và reading order. Export strategy được ADR chốt; nếu không thể thay text an toàn, feature nằm sau capability flag và không xuất hiện trong picker.

## File liên quan

- `docs/decisions/004-pdf-translation-strategy.md`
- `src-tauri/src/document/{mod.rs,model.rs,pdf.rs}`
- `src-tauri/src/storage/export.rs`
- `src-tauri/src/job/orchestrator.rs`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `.github/workflows/ci.yml`
- `tests/fixtures/pdf/**`

## Các bước thực hiện

1. Tạo fixture matrix: Latin/CJK/Việt/Thái/Hàn, multi-column, rotated text, image background, links, scan-only, encrypted và malformed PDF.
2. Spike extraction: đo reading order, Unicode fidelity, bounding boxes và phát hiện scan-only trên cả hai hướng.
3. Spike export: kiểm tra font embedding, page geometry, old-text removal/replacement, searchable text và link preservation.
4. Ghi ADR với lựa chọn, license/packaging impact, fallback và điều kiện disable feature.
5. Triển khai `pdf.rs` theo ADR, resource/time/page limits và cancellation points.
6. Thêm progress `Page N, block M`, overflow warnings và report unsupported pages.
7. Validate temp PDF bằng reopen + page count/page size + translated text extraction trước atomic rename.
8. Tích hợp native asset/binary vào Tauri/CI nếu ADR chọn PDFium; kiểm tra Windows trước rồi macOS/Linux best-effort.
9. Chỉ bật capability khi automated + manual gates pass.

## Todo

- [ ] Không output file khi không có unit dịch được.
- [ ] PDF scan-only trả lỗi OCR chưa hỗ trợ.
- [ ] Font đích hiển thị đúng glyph trên máy không cài font đó.
- [ ] Page count/size/rotation giữ nguyên.
- [ ] Output vẫn searchable/selectable nếu acceptance yêu cầu.
- [ ] License và bundle size được ghi trong ADR/release notes.

## Rủi ro

- Không có chiến lược chung giữ layout hoàn hảo cho mọi PDF; dùng capability gate và phạm vi text-based rõ ràng.
- Rasterize background có thể tăng file size/mất accessibility; chỉ chấp nhận nếu ADR và acceptance criteria cho phép.
- Native PDF engine làm phức tạp release đa nền tảng; CI phải kiểm tra thiếu binary và version mismatch.

## Tiêu chí thành công

- Fixture được hỗ trợ giữ page geometry, render không có missing glyph, text đích trích xuất được và links cơ bản còn hoạt động.
- Fixture không hỗ trợ thất bại rõ ràng trước export, không tạo PDF hỏng/partial.

