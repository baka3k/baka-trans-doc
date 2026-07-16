# Phase 02: Translation core và Ollama

## Bối cảnh

Translation pipeline phải độc lập định dạng để DOCX và PPTX dùng chung retry, chunk, placeholder, prompt và validation.

## Yêu cầu

- Endpoint và model cấu hình được; mặc định Ollama localhost.
- Liệt kê model, health check, timeout và lỗi dễ hiểu.
- Chunk mặc định 1.500–2.000 ký tự, không tách unit nguyên khối như cell/text box nếu có thể.
- Bảo vệ URL, code, số, placeholder và marker style.
- Đầu ra chỉ được apply sau khi validate đầy đủ.

## Kiến trúc

Pipeline thuần:

`normalize → protect → group/chunk → prompt → Ollama → validate → restore`.

Ollama client nằm sau trait `Translator`, cho phép dùng fake translator trong test. Retry policy nhận biết timeout/network, HTTP, malformed marker và cancellation; không retry lỗi cấu hình vĩnh viễn như model không tồn tại.

## File liên quan

- `src-tauri/src/translation/{mod.rs,ollama.rs,prompt.rs,protect.rs,chunk.rs,validate.rs,retry.rs}`
- `src-tauri/src/config.rs`
- `src-tauri/src/error.rs`
- `src-tauri/tests/translation_pipeline.rs`

## Các bước thực hiện

1. Định nghĩa trait `Translator` và request/response không phụ thuộc Ollama.
2. Implement health/model discovery và non-streaming translate request.
3. Xây placeholder registry với ID collision-safe và restore chính xác.
4. Xây chunk planner theo sentence/paragraph boundary; unit vượt ngưỡng được gửi riêng và phát warning.
5. Tạo prompt Nhật → Việt, temperature thấp và yêu cầu giữ marker/paragraph boundary.
6. Validate marker đủ, đúng thứ tự/nesting; phát hiện lời giải thích hoặc output rỗng.
7. Implement exponential backoff có jitter, giới hạn attempt, timeout và cancellation token.
8. Thêm unit/property/integration tests với mock HTTP server.

## Todo

- [ ] Liệt kê được model Ollama cục bộ.
- [ ] Placeholder round-trip 100% trong property tests.
- [ ] Chunk giữ boundary và không làm mất text.
- [ ] Marker sai bị reject trước khi chạm document.
- [ ] Retry/cancel/timeout có test xác định.

## Rủi ro

- Các model nhỏ có thể không giữ marker. Giữ marker ngắn, dễ phân biệt; retry bằng prompt repair; fallback được xử lý ở adapter.
- Bảo vệ mọi số có thể làm bản dịch thiếu tự nhiên. Tách policy thành cấu hình nội bộ và có fixture Nhật–Việt thực tế.

## Tiêu chí thành công

- Translation core chạy bằng fake translator và ít nhất hai model Ollama mà không phụ thuộc DOCX/PPTX.
- Không có output chưa validate nào được trả về như kết quả thành công.

