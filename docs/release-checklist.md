# Release Checklist

## Automated gates

- [ ] `npm ci` hoàn tất và audit không có vulnerability đã biết.
- [ ] `npm run check` pass trên Windows.
- [ ] Structural tests xác nhận ZIP an toàn và part ngoài text không đổi.
- [ ] Không có URL runtime ngoài endpoint Ollama loopback.

## Compatibility

- [ ] Mở mọi golden DOCX/PPTX trong Microsoft Office mà không có repair dialog.
- [ ] Kiểm tra paragraph, table, list, hyperlink, header/footer, textbox, image và page break.
- [ ] Kiểm tra title, shape, table, group shape, theme/master và cảnh báo overflow.
- [ ] Smoke test LibreOffice; ghi nhận khác biệt render nhưng dùng Office làm gate Windows chính.

## Installer

- [ ] Metadata, icon và version đúng.
- [ ] Cài/chạy/gỡ trên Windows sạch.
- [ ] Build public được ký; hoặc ghi rõ internal unsigned build.
- [ ] Hướng dẫn Ollama và known limitations đi kèm release.

## Privacy and recovery

- [ ] Network capture chỉ thấy endpoint loopback đã cấu hình.
- [ ] Cancel không tạo output hoàn tất giả.
- [ ] Resume kiểm tra input/config hash.
- [ ] Checkpoint hoàn tất được xóa và checkpoint dở có thể discard.

