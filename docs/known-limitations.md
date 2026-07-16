# Giới hạn đã biết

## DOCX

- Progress dùng section/paragraph; không cam kết số trang vì không có layout engine.
- Paragraph có Track Changes bị bỏ qua và báo warning.
- Comment, file mã hóa và `.docm` không được hỗ trợ.
- Textbox dựa trên paragraph có thể dịch; biến thể XML hiếm cần thêm fixture.

## PPTX

- Không tự thay font, geometry hoặc autofit. Expansion ratio lớn chỉ tạo cảnh báo overflow.
- SmartArt chỉ được dịch khi text nằm trong diagram data part có mapping an toàn.
- Speaker notes, animation và embedded video không thuộc MVP.
- Slide được nhận diện theo slide part; tài liệu có relationship bất thường cần thêm compatibility fixture.

## Translation

- Chất lượng và khả năng giữ marker phụ thuộc model Ollama.
- Unit nguyên khối vượt giới hạn ký tự không bị cắt giữa cell/textbox; app gửi riêng và cảnh báo.
- File output đã tồn tại không bị ghi đè.

## XLSX

- Chỉ `.xlsx` không macro; `.xls` và `.xlsm` bị từ chối.
- Chỉ shared-string và inline-string cell được dịch. Formula, cached value, số/ngày, sheet name, comment/note, chart, validation, relationship và media được giữ nguyên.
- Shared-string được dùng bởi nhiều cell chỉ dịch một lần. App không tự đổi column width/row height khi text đích dài hơn.

## Markdown

- Chỉ text event hiển thị được dịch bằng source offset; frontmatter, fenced/inline code, URL destination và raw HTML giữ nguyên.
- Cú pháp ngoài các span được thay thế không bị render/normalize lại.

## TXT

- Chỉ hỗ trợ UTF-8. BOM UTF-8, CRLF/LF, dòng trống và khoảng trắng bao quanh từng dòng được giữ nguyên.

## PDF

- PDF chưa được bật trong picker hoặc job runtime vì chưa đạt gate text mapping, Unicode font embedding, searchability và render fidelity đa nền tảng.
- OCR/scan-only, password/DRM, form/XFA và text outline không được hỗ trợ. Xem ADR 004.
