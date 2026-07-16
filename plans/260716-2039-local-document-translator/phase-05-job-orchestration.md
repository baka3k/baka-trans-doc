# Phase 05: Job orchestration, progress và recovery

## Bối cảnh

Tài liệu lớn có thể chạy lâu và một chunk lỗi không được làm hỏng toàn bộ job. Job layer điều phối parser, translation core, checkpoint và exporter mà không block UI.

## Yêu cầu

- Background job, một state machine rõ ràng và cancellation hợp tác.
- Retry rồi skip theo policy; tiếp tục các unit còn lại.
- Progress đơn điệu, ETA dựa trên thời gian unit gần đây.
- Checkpoint/resume an toàn sau crash hoặc app restart.
- Báo cáo cuối có failed/skipped/warning unit và vị trí tài liệu.

## Kiến trúc

Job manifest chứa input hash/metadata, config hash, adapter version, danh sách unit, trạng thái và kết quả đã validate. Checkpoint được ghi atomically theo batch nhỏ hoặc mỗi unit. Resume chỉ cho phép khi input/config tương thích; không reuse output từ cấu hình khác.

Tauri command surface đề xuất:

- `inspect_input`
- `list_models`
- `start_translation`
- `cancel_translation`
- `list_recoverable_jobs`
- `resume_translation`
- `discard_job`

## File liên quan

- `src-tauri/src/job/{mod.rs,orchestrator.rs,state.rs,checkpoint.rs,progress.rs,report.rs}`
- `src-tauri/src/commands/**`
- `src-tauri/src/storage/**`
- `src/types/job.ts`

## Các bước thực hiện

1. Định nghĩa state machine và transition test.
2. Lập kế hoạch unit/chunk trước khi chạy để có total ổn định.
3. Chạy tuần tự mặc định để không quá tải Ollama; giữ interface cho concurrency cấu hình sau.
4. Phát typed progress event sau từng transition/unit.
5. Tính ETA bằng moving average có warm-up; ẩn ETA khi dữ liệu chưa đủ.
6. Ghi checkpoint atomically và dọn checkpoint theo retention policy.
7. Implement cancel tại boundary an toàn và abort HTTP request khi có thể.
8. Tổng hợp failure report JSON nội bộ và bản hiển thị cho người dùng.
9. Test crash simulation, resume, config mismatch, cancellation và partial failure.

## Todo

- [ ] State transition không hợp lệ bị từ chối.
- [ ] Progress không giảm và kết thúc đúng trạng thái.
- [ ] Một unit lỗi không làm mất kết quả unit trước.
- [ ] Resume không dịch lại unit đã validate.
- [ ] Cancel không tạo file output mang trạng thái hoàn tất giả.
- [ ] Report định vị được paragraph/cell/slide/shape lỗi.

## Rủi ro

- Checkpoint chứa text tài liệu cục bộ nhạy cảm. Lưu trong app-data, không log toàn văn mặc định, cho phép discard và dọn sau thành công.
- Resume sau khi code/parser đổi có thể không tương thích. Gắn schema/adapter version và buộc restart khi mismatch.

## Tiêu chí thành công

- Job dài có thể cancel và resume có kiểm soát; mọi trạng thái được phản ánh nhất quán cho UI.
- Partial failure tạo output có warning chỉ khi package vẫn hợp lệ và người dùng được thông báo rõ.

