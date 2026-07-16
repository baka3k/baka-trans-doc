# Phase 06: QA, tài liệu và phát hành

## Bối cảnh

Việc thêm ngôn ngữ và format mở rộng đáng kể matrix lỗi: encoding, font, layout, formula, parser limits, native PDF runtime và checkpoint migration. Release gate phải tách automated invariants khỏi manual compatibility/render smoke.

## Yêu cầu

- Regression đầy đủ cho DOCX/PPTX cùng tests mới cho XLSX/PDF/MD/TXT.
- Resource/path/output safety nhất quán giữa ZIP, text và PDF.
- CI đóng gói được các dependency PDF đã chọn và không có network ngoài Ollama loopback.
- Specs, README, setup, release checklist và known limitations khớp hành vi.

## Kiến trúc

Ba tầng gate: adapter structural/golden tests, application integration/UI tests và manual compatibility/render tests. PDF capability chỉ được bật ở build phát hành khi phase 04 đạt gate; feature flag không được làm ảnh hưởng format khác.

## File liên quan

- `tests/fixtures/**`
- `src-tauri/src/**`
- `src/**/*.test.tsx`
- `.github/workflows/ci.yml`
- `README.md`
- `docs/{specs.md,known-limitations.md,release-checklist.md}`
- `docs/guides/**`
- `src-tauri/tauri.conf.json`

## Các bước thực hiện

1. Xây matrix language scripts × formats, tối thiểu: Latin, Việt, CJK, Hàn và Thái.
2. Chạy adapter contract/golden tests, corrupt/encrypted/oversize/path traversal/output collision cases.
3. Chạy integration fake translator cho success, invalid markers, Ollama down, cancel và resume sau đổi pair/schema.
4. Kiểm tra log/report không chứa document text hoặc raw model response cho mọi format.
5. Thêm CI artifact/smoke cho PDF runtime nếu có; xác minh bundle sạch không phụ thuộc font hệ thống ngoài dự kiến.
6. Manual compatibility: Word/PowerPoint/Excel/LibreOffice; render PDF; mở MD/TXT trong editor giữ encoding/newline.
7. Cập nhật README/UI copy từ DOCX/PPTX và Nhật→Việt sang format/language catalog thực tế.
8. Cập nhật known limitations cho XLSX cell-only, PDF text-only/no OCR, Markdown exclusions và UTF-8 TXT.
9. Chạy `npm run check`, Windows installer smoke và release checklist.

## Todo

- [ ] `npm run check` pass.
- [ ] DOCX/PPTX golden tests không regression.
- [ ] XLSX mở không repair; formula/style/media giữ nguyên.
- [ ] PDF render/text/font gates pass hoặc capability bị tắt rõ ràng.
- [ ] MD/TXT byte-diff chỉ đổi permitted spans.
- [ ] Installer chạy trên Windows sạch và PDF runtime được tìm thấy nếu bật.

## Rủi ro

- Matrix tăng nhanh; giữ fixture nhỏ đại diện trong CI và bộ Office/PDF thực tế trong manual RC suite.
- Model quality khác nhau theo pair không thể xác minh bằng exact string; automated tests dùng fake translator, acceptance Ollama đánh giá cấu trúc và smoke chất lượng riêng.

## Tiêu chí thành công

- Release candidate dịch được mọi format đã bật bằng ít nhất ba language pairs đại diện, không sửa input, không tạo output partial và cung cấp lỗi phục hồi được.
- Tài liệu không hứa OCR, `.xls` hoặc fidelity PDF ngoài capability đã kiểm chứng.

