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

