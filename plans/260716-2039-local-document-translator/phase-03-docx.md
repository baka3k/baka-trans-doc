# Phase 03: Hỗ trợ DOCX

## Bối cảnh

DOCX chứa text ở document, table cell, header/footer, hyperlink và textbox. Mục tiêu là thay text có mapping chính xác, không tái tạo package.

## Yêu cầu

- Đọc paragraph, table cell, header, footer, textbox, list và hyperlink.
- Giữ font, size, emphasis, color, highlight, alignment, spacing, list/numbering, page break, image và relationship.
- Bỏ qua comment và Track Changes theo đặc tả; không hỗ trợ file mã hóa/macro.
- Xuất `{stem}_vi.docx` an toàn.

## Kiến trúc

`DocxAdapter` duyệt các part được phép: `word/document.xml`, `word/header*.xml`, `word/footer*.xml` và textbox lồng trong các part này. Translation unit trỏ về part + paragraph/container + text-node anchors. Table cell vẫn tạo unit riêng nhưng traversal phải tránh extract trùng paragraph.

Run liền kề có cùng effective formatting được gom thành style span. Hyperlink/container boundary luôn là formatting boundary. Writer chỉ thay `w:t`, xử lý `xml:space="preserve"`, và giữ node phi-text như tab/break/drawing.

## File liên quan

- `src-tauri/src/document/model.rs`
- `src-tauri/src/document/docx/{mod.rs,package.rs,extract.rs,styles.rs,replace.rs,validate.rs}`
- `src-tauri/src/storage/{package.rs,export.rs}`
- `tests/fixtures/docx/**`

## Các bước thực hiện

1. Mở ZIP an toàn, kiểm tra content type và part bắt buộc.
2. Discover header/footer part qua relationships thay vì giả định tên cố định.
3. Extract leaf paragraphs/cells/textbox/hyperlink thành unit có vị trí dùng cho progress/report.
4. Gom style span và tạo marker cho inline formatting khác biệt.
5. Apply bản dịch vào anchor; giữ break/tab/drawing và khoảng trắng XML.
6. Implement fallback theo formatting island khi marker retry thất bại, kèm warning.
7. Đóng gói ra temp, validate ZIP/part/relationship, atomic rename sang `_vi.docx`.
8. Xây fixture matrix: plain, multi-run, list, table, nested table, image, link, header/footer, textbox, page break, CJK font.
9. Golden diff và Office/LibreOffice smoke test.

## Todo

- [ ] Mỗi loại nội dung bắt buộc có fixture và assertion.
- [ ] Media hash và relationship không đổi.
- [ ] Numbering/list vẫn hoạt động sau dịch.
- [ ] Inline bold/italic/color/hyperlink được giữ.
- [ ] File lỗi/corrupt/encrypted trả lỗi có mã và không để output rác.

## Rủi ro

- Textbox có thể nằm trong VML hoặc DrawingML khác nhau; cần fixture từ nhiều phiên bản Office.
- Effective style có thể kế thừa từ style/theme. Bản đầu không cần resolve toàn bộ style để giữ nguyên node, nhưng cần nhận biết run-property boundary khi đặt marker.
- Track Changes có thể chứa text hiển thị. Theo scope, bỏ qua node revision và báo warning thay vì dịch một phần âm thầm.

## Tiêu chí thành công

- Bộ DOCX fixture mở được, nội dung mục tiêu được dịch, mọi invariant ngoài text được giữ.
- Unsupported content xuất hiện trong report rõ ràng, không làm hỏng phần còn lại.

