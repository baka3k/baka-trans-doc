# Phase 05: UI, progress và recovery

## Bối cảnh

`src/app/App.tsx` hiện render cặp Nhật → Việt cố định, icon chỉ phân biệt Word/PowerPoint và footer ghi DOCX/PPTX. `src/lib/tauri.ts` cũng giới hạn native picker ở hai extension.

## Yêu cầu

- Hai select source/target lấy từ backend catalog, có label native + display name.
- Không cho start khi pair trùng/thiếu hoặc format capability chưa sẵn sàng.
- Picker, icon, inspection summary, output preview và footer phản ánh format mới.
- Recovery hiển thị language pair/format; checkpoint không tương thích có CTA discard rõ ràng.
- Progress/location dùng label của adapter, không giả định paragraph/slide.

## Kiến trúc

Frontend thêm typed `LanguageInfo`, `DocumentCapabilities` và state cấu hình. Backend vẫn validate toàn bộ request. Output preview dùng helper/field do backend sinh theo target language để không lệch naming rule. UI hiển thị warning format-specific ngay sau inspection, đặc biệt PDF text-only và XLSX cell-only.

## File liên quan

- `src/app/{App.tsx,styles.css}`
- `src/components/{ProgressPanel.tsx,RecoveryBanner.tsx}`
- `src/lib/tauri.ts`
- `src/types/{document.ts,job.ts}`
- `src/store/{job.ts,job.test.ts}`
- `src/**/*.test.tsx`

## Các bước thực hiện

1. Thêm wrapper `listLanguages`, typed catalog/capabilities và request fields.
2. Thay language row cố định bằng hai accessible selects; giữ lựa chọn khi đổi file, chặn pair trùng.
3. Mở rộng native file filter và mapping icon/label cho XLSX/PDF/MD/TXT.
4. Hiển thị inspection warnings/capability notes; PDF chưa qua gate không xuất hiện hoặc disabled với lý do.
5. Tạo output preview theo target code và extension gốc; cập nhật khi đổi target.
6. Gửi language pair khi start; render pair/format trong recovery và preflight summary.
7. Generalize progress copy/current item; giữ event filtering và percent monotonic hiện có.
8. Cập nhật responsive layout, keyboard/focus/error announcement cho hai select và warning blocks.
9. Thêm component tests cho catalog load/error, swap pair, same-language, format filters, start payload và recovery.

## Todo

- [ ] Select có label/accessibility và không dùng flag làm định danh duy nhất.
- [ ] Pair trùng chặn start với thông báo cụ thể.
- [ ] Payload start chứa canonical codes.
- [ ] PDF/XLSX limitations hiển thị trước khi bắt đầu.
- [ ] UI không assume mọi current item là slide/paragraph.

## Rủi ro

- Catalog load lỗi có thể làm form kẹt; có retry và không dùng fallback khác backend.
- Picker filter không thay thế backend validation; file kéo/thả/đường dẫn vẫn phải inspect ở Rust.

## Tiêu chí thành công

- Người dùng hoàn tất job cho mỗi format được bật với pair bất kỳ trong catalog mà không sửa config/terminal.
- UI, request, progress, recovery và output name hiển thị cùng language pair/format.

