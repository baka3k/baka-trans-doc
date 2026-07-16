# Phase 04: Hỗ trợ PPTX

## Bối cảnh

PPTX có text trong shape, title, textbox, table và có thể trong diagram/SmartArt part. Tiếng Việt dài hơn dễ gây overflow, dù geometry/theme vẫn được giữ.

## Yêu cầu

- Đọc title, shape/textbox và table cell trên từng slide.
- Đọc text SmartArt khi có mapping OOXML an toàn; speaker notes là opt-in sau MVP.
- Giữ slide layout, master, theme, image, relationship, geometry, table style và run formatting.
- Progress xác định chính xác slide và shape/unit hiện tại.

## Kiến trúc

`PptxAdapter` discover slide theo presentation relationships, không dựa vào sort tên file. Mỗi `a:p` trong shape/table cell là unit; style span map về `a:r/a:t`. Diagram parts được discover qua relationships và chỉ sửa text part đã xác định.

Mặc định không thay font size, shape bounds hay autofit để tránh biến đổi layout không dự đoán được. Adapter ghi metric expansion và cờ nguy cơ overflow vào report.

## File liên quan

- `src-tauri/src/document/pptx/{mod.rs,package.rs,extract.rs,styles.rs,replace.rs,diagram.rs,validate.rs}`
- `src-tauri/src/storage/{package.rs,export.rs}`
- `tests/fixtures/pptx/**`

## Các bước thực hiện

1. Discover presentation, slide order, slide/diagram relationships và content types.
2. Extract `a:p/a:r/a:t` từ title, shape, textbox, group shape và table cell.
3. Map unit sang slide number + shape ID + paragraph index cho report/progress.
4. Gom style span, bảo vệ hyperlink/action boundary và tạo marker.
5. Apply text tối thiểu, giữ run properties, paragraph properties và geometry.
6. Spike SmartArt với fixture; hỗ trợ part map được, cảnh báo phần không truy cập an toàn.
7. Tính expansion ratio/ước lượng overflow; không tự chỉnh layout trong MVP.
8. Đóng gói/validate/atomic export thành `_vi.pptx`.
9. Golden diff và Office/LibreOffice smoke test.

## Todo

- [ ] Slide order và số slide trong progress chính xác.
- [ ] Shape/title/table/group fixture được dịch.
- [ ] Theme/master/layout/media/rels không đổi.
- [ ] Inline style và hyperlink được giữ.
- [ ] SmartArt supported/unsupported được báo minh bạch.
- [ ] Overflow risk có warning theo unit.

## Rủi ro

- SmartArt phức tạp và có duplicated text/cache. Chỉ ghi vào source-of-truth part đã xác nhận bằng fixture.
- LibreOffice render khác PowerPoint; Microsoft Office là compatibility gate chính cho Windows-first.
- Expansion ratio chỉ là heuristic, không thay thế layout engine.

## Tiêu chí thành công

- Tất cả slide fixture mở được và giữ cấu trúc/theme/media.
- App không tuyên bố thành công im lặng khi có text không thể map hoặc có nguy cơ overflow cao.

