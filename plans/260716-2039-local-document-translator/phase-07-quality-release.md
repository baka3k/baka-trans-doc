# Phase 07: QA, hardening và phát hành

## Bối cảnh

Giá trị của app phụ thuộc vào việc output mở được và không phá tài liệu. Phase cuối tập trung vào compatibility, security/offline invariant, hiệu năng và installer Windows.

## Yêu cầu

- Test matrix đủ cho DOCX/PPTX thực tế và failure modes.
- Xác nhận không có cloud traffic/telemetry ngoài endpoint Ollama cấu hình.
- Xử lý ZIP/XML an toàn, giới hạn tài nguyên và path traversal.
- Đóng gói installer Windows có hướng dẫn Ollama/model.
- Có checklist release và known limitations.

## Kiến trúc

Tách ba tầng gate: structural package tests, application integration tests và manual compatibility/render smoke. Benchmark theo số unit/kích thước file, không chỉ tổng ký tự. Parser phải giới hạn kích thước decompressed, số part và độ sâu XML để tránh zip bomb/resource exhaustion.

## File liên quan

- `tests/fixtures/**`
- `src-tauri/tests/**`
- `src/**/*.test.tsx`
- `.github/workflows/**`
- `docs/guides/**`
- `docs/release-checklist.md`
- Tauri bundle/signing configuration

## Các bước thực hiện

1. Mở rộng fixture matrix bằng tài liệu sinh tự động và tài liệu Office thực tế đã loại dữ liệu nhạy cảm.
2. Thêm package invariants và snapshot/diff tooling dễ đọc.
3. Chạy integration matrix: model success, model missing, Ollama down, timeout, malformed output, disk full, read-only output, corrupt input, cancel/resume.
4. Thêm parser limits, safe ZIP paths, output-path checks và redaction log.
5. Benchmark tài liệu nhỏ/vừa/lớn; tối ưu memory bằng streaming part khi cần nhưng không hy sinh tính đúng.
6. Smoke test Windows + Microsoft Office là release gate; Linux/macOS là best-effort trong MVP.
7. Đóng gói installer, version metadata, icon, license/notice và hướng dẫn cài Ollama/model.
8. Viết known limitations: DOCX page progress, overflow, SmartArt, unsupported revisions/encryption.
9. Chạy release checklist và tạo release candidate.

## Todo

- [ ] Structural/integration/UI test pass trong CI.
- [ ] Office mở mọi golden output không repair dialog.
- [ ] Không có request tới host ngoài endpoint cấu hình trong network test.
- [ ] ZIP bomb/path traversal/resource limit có test.
- [ ] Installer sạch cài/chạy/gỡ được trên Windows test machine.
- [ ] Hướng dẫn troubleshooting Ollama và model rõ ràng.

## Rủi ro

- Signing certificate có thể chưa sẵn sàng; tách unsigned internal build và signed public release.
- Test render tự động không giống Office. Giữ manual Office smoke checklist cho RC đầu.
- Model lớn gây memory pressure thuộc Ollama, nhưng app vẫn cần timeout/cancel và thông báo tài nguyên dễ hiểu.

## Tiêu chí thành công

- Release candidate cài được trên Windows sạch, dịch được bộ acceptance fixture và output mở không cảnh báo repair.
- Known limitations và failure report trung thực, đủ để người dùng phục hồi hoặc thử lại.
