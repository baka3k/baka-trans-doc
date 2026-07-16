# Phase 02: Lõi đa định dạng, Markdown và TXT

## Bối cảnh

`DocumentPackage` hiện luôn mở `OoxmlPackage`, apply vào ZIP và trả lại package. Contract này không phù hợp file text hoặc PDF và khiến orchestrator kiểm tra extension trực tiếp.

## Yêu cầu

- Orchestrator không biết chi tiết storage/extension của adapter.
- Mọi adapter cung cấp inspect, units, warnings, apply translation và atomic export.
- Markdown/TXT giữ cấu trúc nguồn ngoài các text span được phép dịch.
- Resource limit áp dụng cho cả file không phải ZIP.

## Kiến trúc

Đổi `DocumentPackage` thành `DocumentSession` format-neutral với enum representation nội bộ (`Ooxml`, `Markdown`, `Text`, về sau `Pdf`). Dùng enum trước trait object để giữ ownership/error flow rõ ràng. Tách atomic path validation/temp/rename khỏi `export_atomic(OoxmlPackage)`; từng representation ghi và tự validate temp output qua callback chung.

Markdown dùng parser có source offsets; chỉ tạo units từ heading/paragraph/list/table/link-label/image-alt text, bỏ qua fenced/inline code, URL destination, frontmatter và raw HTML. TXT đọc UTF-8, giữ BOM/newline convention và map unit theo paragraph hoặc nhóm dòng không trống.

## File liên quan

- `src-tauri/src/document/{mod.rs,model.rs,markdown.rs,text.rs}`
- `src-tauri/src/storage/{export.rs,package.rs}`
- `src-tauri/src/job/orchestrator.rs`
- `src-tauri/src/error.rs`
- `src-tauri/Cargo.toml`
- `tests/fixtures/{markdown,text}/**`

## Các bước thực hiện

1. Định nghĩa `DocumentKind` mở rộng, `DocumentCapabilities` và `DocumentSession` API.
2. Chuyển DOCX/PPTX sang representation OOXML mới mà không đổi hành vi/golden tests.
3. Tách safe output path, sibling temp, no-overwrite, validation và atomic rename thành exporter dùng chung.
4. Thêm giới hạn byte cho MD/TXT và lỗi encoding rõ ràng.
5. Xây Markdown span extractor/rewriter; apply replacements từ offset lớn về nhỏ để không lệch mapping.
6. Xây TXT extractor/rewriter, giữ BOM, CRLF/LF và blank lines.
7. Chuyển job progress/location sang label adapter cung cấp; bỏ helper kiểm tra extension trong orchestrator.
8. Thêm contract tests chạy trên DOCX/PPTX/MD/TXT và corrupt/oversize fixtures.

## Todo

- [ ] DOCX/PPTX regression tests giữ nguyên.
- [ ] Markdown code, URL, frontmatter và syntax không đổi byte.
- [ ] TXT CRLF/LF, BOM/no-BOM và Unicode đa script được giữ.
- [ ] Export failure xóa temp và không tạo partial output.
- [ ] Input và existing output không bao giờ bị overwrite.

## Rủi ro

- Parser Markdown có thể normalize syntax nếu serialize AST; phải rewrite theo source offsets, không render lại toàn bộ AST.
- Paragraph TXT rất lớn vẫn là atomic unit; giữ warning hiện tại thay vì cắt mất cấu trúc.

## Tiêu chí thành công

- Orchestrator chạy cùng một flow cho bốn kind mà không `match` extension.
- Golden diff của MD/TXT chỉ chứa text spans mong đợi và output có thể đọc lại qua adapter.

