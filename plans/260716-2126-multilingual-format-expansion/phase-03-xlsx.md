# Phase 03: Hỗ trợ XLSX

## Bối cảnh

XLSX cùng là OOXML ZIP nên có thể tái sử dụng package safety và XML marker logic, nhưng text được lưu qua shared strings, inline strings và rich-text runs; formula/numeric/date cells không được dịch.

## Yêu cầu

- Chỉ hỗ trợ `.xlsx` không macro/encryption; từ chối `.xls` và `.xlsm` rõ ràng.
- Dịch string cell từ `sharedStrings.xml` và `inlineStr`.
- Giữ formula, cached formula result, number/date, style index, validation, merged cells, workbook relationships và media.
- Rich text giữ run formatting qua style markers.
- Progress label theo `SheetName!Cell` khi map an toàn.

## Kiến trúc

Thêm `document/xlsx.rs` dùng `OoxmlPackage`. Parse workbook relationships để map worksheet part → sheet name, worksheet cells → shared string index/inline string. Shared-string record là đơn vị mutation; lập reverse reference map để gắn location đại diện và tránh dịch cùng record nhiều lần. Chỉ sửa `<t>` trong `<si>` hoặc `<is>`, tái sử dụng marker validation cho rich runs.

## File liên quan

- `src-tauri/src/document/{mod.rs,model.rs,xlsx.rs,xml.rs}`
- `src-tauri/src/storage/package.rs`
- `src-tauri/src/job/orchestrator.rs`
- `src/types/document.ts`
- `tests/fixtures/xlsx/**`
- `docs/known-limitations.md`

## Các bước thực hiện

1. Validate content type/workbook parts và từ chối macro/encrypted/legacy formats.
2. Parse workbook + relationships + worksheet cell references với path resolution an toàn.
3. Extract shared strings, inline strings và rich runs; bỏ qua formula/numeric/boolean/error/blank cells.
4. Deduplicate translation units theo shared-string record, giữ danh sách cell references cho report.
5. Apply text vào đúng `<t>` nodes, giữ `xml:space`, run properties và mọi part không liên quan.
6. Thêm overflow warning theo expansion ratio/cell wrap state nhưng không tự đổi column width/row height.
7. Thêm fixtures: shared/inline/rich text, formula, date/number, merged cells, hidden sheet, image và corrupt workbook.
8. Golden test hash formula/style/media/relationship parts và mở output bằng Excel/LibreOffice trong manual gate.

## Todo

- [ ] Formula và cached value không đổi.
- [ ] Shared string dùng bởi nhiều cell chỉ dịch một lần.
- [ ] Rich text giữ style runs/marker order.
- [ ] Sheet name/location không làm lộ document text trong log.
- [ ] XLSX output mở không có repair dialog.

## Rủi ro

- Shared string dùng ở nhiều ngữ cảnh có thể cần bản dịch khác nhau; bản đầu dịch nhất quán một record và báo references trong report.
- Excel có text trong drawing/chart/comment ngoài cell; để ngoài phạm vi và ghi known limitation.

## Tiêu chí thành công

- Workbook fixture thực tế dịch đúng string cells, không sửa formula/number/date/style/media và mở được trong Excel/LibreOffice.

