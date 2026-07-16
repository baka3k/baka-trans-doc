# Phase 06: Giao diện desktop

## Bối cảnh

UI phải làm rõ trạng thái Ollama, input/output, tiến độ và kết quả mà không yêu cầu người dùng hiểu cấu trúc OOXML.

## Yêu cầu

- Chọn input DOCX/PPTX và output folder bằng native dialog.
- Chọn model Ollama; source/target hiển thị Nhật/Việt và khóa theo MVP.
- Validate trước khi bắt đầu, hiển thị progress/current item/ETA/log.
- Cancel, resume/restart job recoverable và mở thư mục kết quả.
- Responsive trong kích thước cửa sổ desktop hợp lý; keyboard accessible.

## Kiến trúc

Store frontend giữ state UI/job đã typed, nhưng backend là source of truth cho job. Event listener được đăng ký/hủy đúng lifecycle và bỏ qua event không cùng job ID. Log hiển thị structured message đã sanitize, không render raw model response.

## File liên quan

- `src/app/**`
- `src/pages/TranslatePage.tsx`
- `src/components/{FilePicker,ModelSelect,LanguagePair,ProgressPanel,JobLog,ResultPanel,RecoveryDialog}.tsx`
- `src/store/**`
- `src/lib/tauri.ts`
- `src/types/**`

## Các bước thực hiện

1. Xây typed command/event wrapper và store state machine.
2. Làm input/output picker, extension validation và output-name preview.
3. Làm Ollama status/model select với empty/error/retry states.
4. Làm Translate action với preflight summary.
5. Làm progress panel: phase, percent, slide hoặc paragraph/section, ETA và cancel.
6. Làm log/warning list có filter, copy report và không lộ toàn văn tài liệu mặc định.
7. Làm completion/result panel: output path, warning count, open folder.
8. Làm recovery dialog khi app phát hiện job dở.
9. Thêm component/integration tests cho happy path và error states.

## Todo

- [ ] Không cho start khi input/model/output chưa hợp lệ.
- [ ] UI không freeze khi dịch.
- [ ] DOCX không hiển thị page giả; dùng paragraph/section.
- [ ] Cancel/recovery/error có CTA rõ ràng.
- [ ] Keyboard navigation, focus và color contrast đạt mức cơ bản.
- [ ] File nguồn và file output được phân biệt rõ.

## Rủi ro

- Event đến trễ từ job cũ làm hỏng UI. Mọi reducer action phải kiểm tra job ID và transition.
- Log quá nhiều làm chậm render. Giới hạn buffer UI/virtualize, còn report đầy đủ lưu theo job nếu cần.

## Tiêu chí thành công

- Người dùng mới có thể hoàn tất một job mà không dùng terminal.
- Mọi trạng thái backend quan trọng có biểu diễn UI và hướng xử lý rõ ràng.

