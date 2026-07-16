---
title: "Kế hoạch triển khai Local Document Translator"
status: implementation_complete
created: 2026-07-16
source: docs/specs.md
blockedBy: []
validation_status: manual_gates_pending
---

# Kế hoạch triển khai Local Document Translator

## Tổng quan

Xây dựng ứng dụng desktop offline, Windows-first và có khả năng mở rộng đa nền tảng bằng Tauri. Phiên bản đầu dịch Nhật → Việt qua Ollama cục bộ, hỗ trợ DOCX và PPTX, đồng thời ưu tiên không làm thay đổi cấu trúc, media, relationship, style và layout của tài liệu.

Repo hiện chỉ có đặc tả, chưa có source code, test fixture, plan đang hoạt động hoặc `docs/development-rules.md`. Kế hoạch vì vậy bao gồm cả bootstrap dự án, kiến trúc lõi, vertical slice giảm rủi ro, triển khai hai định dạng, UI và hardening.

## Kết quả triển khai — 2026-07-16

Đã hoàn thành phần triển khai MVP: scaffold Tauri 2/React/TypeScript, Rust domain core, Ollama client, placeholder/marker validation, DOCX/PPTX OOXML adapters, safe ZIP/export limits, background job/cancel/checkpoint/resume, typed progress events, giao diện desktop, CI, ADR, fixture sources và Windows bundling.

Đã xác minh:

- `npm run check` pass: ESLint, 2 frontend tests, TypeScript/Vite production build, `cargo fmt`, Clippy với warnings bị từ chối và 9 Rust tests.
- Golden structural test giữ nguyên media/content-types byte và chỉ thay mapped text node, kể cả multi-run bold.
- Release build tạo thành công MSI và NSIS; executable đóng gói khởi chạy được trong smoke test.
- Ollama local có model `translategemma:4b`; API smoke chạy được. Model này đổi style marker trong prompt thử trực tiếp, đúng failure mode mà pipeline validator/retry/skip phải chặn.

Các release gate thủ công còn lại:

- Mở bộ fixture thực tế trong Microsoft Office/LibreOffice và xác nhận không có repair dialog hoặc layout regression.
- Acceptance test với model Ollama thứ hai; máy hiện chỉ có một model.
- Mở rộng fixture matrix cho table, list, hyperlink, header/footer, textbox, group shape và SmartArt từ tài liệu Office thật đã loại dữ liệu nhạy cảm.
- Cài/gỡ installer trên Windows sạch và ký code cho public release.

## Phạm vi phiên bản đầu

### Bao gồm

- Tauri 2, Rust backend, React + TypeScript + Vite frontend.
- Kết nối Ollama tại endpoint cấu hình được, mặc định `http://localhost:11434`.
- Liệt kê và chọn model đã có trong Ollama.
- Dịch DOCX và PPTX Nhật → Việt theo từng translation unit/chunk.
- Bảo vệ URL, code, số và marker định dạng trước khi gửi model.
- Retry, bỏ qua unit lỗi có kiểm soát, hủy job, ghi checkpoint và báo cáo cuối.
- Progress theo paragraph/section đối với DOCX và slide đối với PPTX; hiển thị phần trăm và ETA.
- Xuất file mới theo hậu tố `_vi`, không sửa file nguồn.

### Chưa bao gồm

- PDF, XLSX, Markdown, batch, OCR, translation memory, glossary, bilingual output.
- Track Changes, comments, macro, animation và embedded video.
- Streaming preview; API Ollama có thể stream nội bộ nhưng UI bản đầu chỉ cần progress theo unit.
- Speaker notes mặc định; thiết kế parser để có thể bật sau.

## Quyết định kiến trúc

### 1. Chỉnh trực tiếp OOXML thay vì dựng lại tài liệu

DOCX/PPTX là gói ZIP chứa XML và media. Backend sao chép gói nguồn, chỉ sửa nội dung các node text (`w:t`, `a:t`) cần dịch, sau đó đóng gói lại. Các part khác, relationship, image, theme, numbering và style được giữ nguyên byte khi có thể. Cách này giảm mạnh nguy cơ mất định dạng so với chuyển tài liệu sang mô hình trung gian rồi tạo lại từ đầu.

### 2. Domain core độc lập với Tauri

Tách `document`, `translation`, `job` và `storage` thành các module Rust không phụ thuộc UI. Tauri command chỉ validate input, khởi chạy job và phát event. Nhờ vậy logic có thể unit/integration test mà không cần mở desktop window.

### 3. Translation unit có mapping về XML nguồn

Mỗi unit chứa định danh ổn định, loại tài liệu/vị trí, plain text, các đoạn style liền kề, placeholder và danh sách node đích. Các đoạn có cùng style được gom lại. Khi inline style khác nhau, chèn marker bất biến vào prompt; đầu ra phải qua validator trước khi ghi ngược vào đúng run.

Nếu model làm mất marker: retry bằng prompt chặt hơn; nếu vẫn lỗi, dịch từng “formatting island” và ghi warning. Không âm thầm làm mất bold/italic/hyperlink.

### 4. Job bền vững và xuất file an toàn

Mỗi job có manifest/checkpoint trong app-data, ghi kết quả ra file tạm cùng filesystem với output, xác thực gói ZIP/OOXML cơ bản rồi atomic rename. Input không bao giờ bị overwrite. Job có trạng thái `queued/running/cancelling/completed/completed_with_warnings/failed`.

### 5. Event progress có schema ổn định

Backend phát typed events gồm job ID, phase, unit hiện tại, tổng unit, percent, ETA, warning và log level. UI không suy diễn progress từ log string.

## Kiến trúc mục tiêu

```text
React/TypeScript UI
  ├─ file/model/settings form
  ├─ job progress + cancel
  └─ result + failure report
          │ Tauri invoke/events
Rust application layer
  ├─ commands (thin adapters)
  ├─ job orchestrator + checkpoint
  ├─ document abstraction
  │    ├─ DOCX OOXML adapter
  │    └─ PPTX OOXML adapter
  ├─ translation pipeline
  │    ├─ normalize/protect/chunk
  │    ├─ Ollama client
  │    └─ validate/restore
  └─ safe exporter
          │ localhost HTTP
        Ollama
```

## Cấu trúc source dự kiến

```text
src/
  app/
  pages/
  components/
  store/
  lib/
  types/
src-tauri/src/
  commands/
  document/
    docx/
    pptx/
    model.rs
  translation/
    ollama.rs
    prompt.rs
    protect.rs
    chunk.rs
    validate.rs
  job/
    orchestrator.rs
    checkpoint.rs
    progress.rs
  storage/
    package.rs
    export.rs
  error.rs
  config.rs
tests/fixtures/
  docx/
  pptx/
docs/decisions/
```

Tên file thực tế có thể điều chỉnh theo scaffold Tauri, nhưng ranh giới module phải được giữ.

## Các giai đoạn

| Phase | Mục tiêu | Đầu ra chính | Phụ thuộc |
| --- | --- | --- | --- |
| 01 | Bootstrap và vertical slice | App chạy được, ADR, dịch một paragraph fixture end-to-end | Không |
| 02 | Translation core + Ollama | Client, protect/chunk/prompt/validate, retry | Phase 01 |
| 03 | DOCX đầy đủ | Parser/rewriter/exporter DOCX và golden fixtures | Phase 02 |
| 04 | PPTX đầy đủ | Parser/rewriter/exporter PPTX và golden fixtures | Phase 02 |
| 05 | Job, progress và recovery | Background job, cancel, checkpoint, report | Phase 02, 03, 04 |
| 06 | UI hoàn chỉnh | Chọn file/model/output, progress, log, result | Phase 01, 05 |
| 07 | QA, hardening và đóng gói | Test matrix, compatibility, installer Windows | Phase 03–06 |

Chi tiết:

- [Phase 01: Nền tảng và vertical slice](phase-01-foundation.md)
- [Phase 02: Translation core và Ollama](phase-02-translation-core.md)
- [Phase 03: Hỗ trợ DOCX](phase-03-docx.md)
- [Phase 04: Hỗ trợ PPTX](phase-04-pptx.md)
- [Phase 05: Job orchestration, progress và recovery](phase-05-job-orchestration.md)
- [Phase 06: Giao diện desktop](phase-06-ui.md)
- [Phase 07: QA, hardening và phát hành](phase-07-quality-release.md)

## Phụ thuộc và đường găng

`Phase 01 → Phase 02 → (Phase 03 || Phase 04) → Phase 05 → Phase 06 → Phase 07`.

DOCX và PPTX có thể phát triển song song sau khi translation core và contract `DocumentAdapter` ổn định. Phase 05 nên dùng fixture/mocked adapter từ sớm để không chờ cả hai parser hoàn tất.

Phụ thuộc hệ thống phát triển:

- Rust toolchain, Node.js, package manager được pin bằng lockfile.
- Tauri prerequisites theo từng OS; Windows cần WebView2 và toolchain phù hợp.
- Ollama chạy cục bộ và có ít nhất một model test.
- Microsoft Office hoặc LibreOffice cho smoke test mở file; test cấu trúc không được phụ thuộc hoàn toàn vào ứng dụng Office.

## Chiến lược kiểm thử

- Unit test: normalize, placeholder, chunk boundary, marker validation, ETA, retry policy.
- Property/fuzz test: placeholder luôn restore được, chunk không vượt ngưỡng ngoài ngoại lệ unit nguyên khối, không tạo marker trùng.
- Golden-package test: media hash không đổi, relationship/theme/style không đổi, chỉ text node dự kiến thay đổi.
- Integration test: mock Ollama HTTP cho success, timeout, malformed output, model missing và cancellation.
- Compatibility smoke: mở/sửa/lưu file kết quả trong Office và LibreOffice trên bộ fixture có table, image, list, hyperlink, textbox, header/footer, theme.
- UI test: form validation, progress state, cancel, warning report và đường dẫn output.

## Rủi ro chính và biện pháp

| Rủi ro | Mức | Biện pháp |
| --- | --- | --- |
| LLM làm mất/đổi marker inline format | Cao | Validator nghiêm ngặt, retry, fallback theo formatting island, report warning |
| Tiếng Việt dài hơn gây overflow/reflow | Cao | Không sửa geometry mặc định; phát hiện shape/text có nguy cơ; smoke/render QA; cân nhắc autofit opt-in sau MVP |
| DOCX không có “page hiện tại” đáng tin nếu không render | Trung bình | UI hiển thị section/paragraph cho DOCX; chỉ hiển thị page khi có page-break mapping rõ ràng |
| SmartArt lưu text ngoài slide XML | Trung bình | Spike part discovery; hỗ trợ text truy cập được, fixture riêng; báo unsupported khi không map an toàn |
| XML writer làm thay đổi namespace/whitespace ngoài ý muốn | Cao | Ưu tiên mutation tối thiểu, canonical diff test, giữ nguyên part không sửa, test Office compatibility |
| Job dài bị crash/mất điện | Cao | Checkpoint theo unit/chunk, output tạm, resume dựa trên input hash + config hash |
| Model Ollama khác nhau tuân thủ prompt khác nhau | Trung bình | Capability-agnostic prompt, temperature thấp, validation đầu ra, model-specific setting chỉ là override |

## Giả định đã chốt cho kế hoạch

- Bản đầu tối ưu Windows nhưng không dùng Office COM/Interop, để giữ khả năng cross-platform và offline.
- Chỉ hỗ trợ file OOXML không mã hóa; file password-protected trả lỗi rõ ràng.
- File có macro (`.docm`, `.pptm`) không nằm trong phạm vi, tránh rủi ro làm hỏng hoặc vô tình xử lý macro.
- UI hiển thị “paragraph/section” cho DOCX thay vì cam kết page tuyệt đối khi không có layout engine.
- Bản đầu không tự đổi font hoặc geometry để chữa overflow; ưu tiên bảo toàn style/layout source và báo cảnh báo.

## Tiêu chí thành công toàn dự án

- Dịch thành công bộ fixture DOCX/PPTX Nhật → Việt qua ít nhất hai model Ollama phổ biến.
- Output mở được trong Microsoft Office; input không bị thay đổi; tên file tuân thủ hậu tố `_vi`.
- Image/media và relationship hash giữ nguyên; style, numbering, theme và slide/page structure không bị tái tạo.
- Bold/italic/underline/color/hyperlink/list/table được giữ trong các fixture bắt buộc.
- Chunk lỗi không làm mất toàn bộ job; báo cáo liệt kê chính xác unit thất bại và warning.
- Cancel dừng an toàn; app đóng/mở lại có thể phát hiện job dở và resume/restart có kiểm soát.
- UI không bị block trong suốt job và progress tăng đơn điệu tới trạng thái kết thúc.
- Toàn bộ xử lý và dữ liệu vẫn nằm trên máy; chỉ kết nối endpoint Ollama đã cấu hình.

## Definition of Done cho mỗi phase

- Code được format/lint, test liên quan chạy pass.
- Có fixture/test cho hành vi mới và lỗi quan trọng.
- Contract/event/schema thay đổi được cập nhật trong docs hoặc ADR.
- Không làm giảm các invariant: không overwrite input, không gửi dữ liệu ra cloud, không âm thầm làm mất nội dung/định dạng.
