---
title: "Mở rộng đa ngôn ngữ và định dạng tài liệu"
status: implemented
created: 2026-07-16
implemented: 2026-07-16
blockedBy: []
buildsOn:
  - 260716-2039-local-document-translator
---

# Mở rộng đa ngôn ngữ và định dạng tài liệu

## Kết quả triển khai

- Đã triển khai catalog ngôn ngữ backend, prompt động, output suffix canonical, checkpoint schema 2 và recovery metadata.
- Đã chuyển orchestration sang `DocumentSession`; DOCX/PPTX giữ flow OOXML, Markdown/TXT dùng source-offset/byte-preserving exporter và XLSX dịch shared/inline strings.
- Đã tích hợp language selects, capability/limitation, file picker và recovery UI cho các format được bật.
- PDF có typed capability nhưng bị tắt theo ADR 004 vì chưa đạt gate mapping/font/render; picker production không quảng bá PDF và runtime không tạo output PDF.
- `npm run check` pass với 3 frontend tests và 20 Rust tests. Manual Office/LibreOffice/installer compatibility vẫn thuộc release checklist trước khi phát hành binary.

## Tổng quan

Mở rộng Local Document Translator từ cặp Nhật → Việt và DOCX/PPTX cố định thành ứng dụng cho phép chọn ngôn ngữ nguồn, ngôn ngữ đích và dịch thêm XLSX, PDF có text, Markdown và TXT. Thay đổi phải giữ các invariant hiện có: xử lý hoàn toàn cục bộ qua Ollama, không ghi đè input, checkpoint/resume an toàn, progress typed và không âm thầm làm mất cấu trúc tài liệu.

Code hiện tại khóa ngôn ngữ tại `src-tauri/src/translation/prompt.rs`, chưa truyền language pair qua `StartTranslationRequest`/`TranslationConfig`, dùng hậu tố `_vi` trong `src-tauri/src/document/mod.rs` và `src-tauri/src/job/orchestrator.rs`, đồng thời gắn `DocumentPackage` trực tiếp với `OoxmlPackage`. Frontend cũng khóa Nhật → Việt trong `src/app/App.tsx` và chỉ chọn `.docx/.pptx` trong `src/lib/tauri.ts`. Vì vậy cần mở rộng contract lõi trước khi thêm adapter mới.

## Phạm vi

### Bao gồm

- Chọn tường minh ngôn ngữ nguồn và đích từ catalog backend; không cho chọn cùng một ngôn ngữ.
- Catalog ban đầu: Việt, Nhật, Anh, Trung giản thể, Trung phồn thể, Hàn, Thái, Pháp, Đức và Tây Ban Nha; dùng mã BCP 47 ổn định trong request/checkpoint.
- Prompt động theo cặp ngôn ngữ, vẫn giữ placeholder/style marker và chỉ trả nội dung dịch.
- Tên output dùng mã ngôn ngữ đích, ví dụ `report_en.docx`, thay cho `_vi` cố định.
- Hỗ trợ input/output cùng định dạng cho `.docx`, `.pptx`, `.xlsx`, `.pdf`, `.md`, `.markdown` và `.txt`.
- XLSX dịch text trong cell/rich text, giữ công thức, số, ngày, style, sheet structure và media.
- Markdown dịch text hiển thị, giữ syntax, URL, code, frontmatter và raw HTML nguyên trạng.
- TXT UTF-8 giữ BOM nếu có, kiểu newline và khoảng trắng/đoạn trống.
- PDF có text, không mã hóa; giữ page size/page count và kiểm soát overflow theo chiến lược được chốt qua ADR.
- Progress/location phù hợp từng định dạng và checkpoint chứa language pair + adapter version.

### Chưa bao gồm

- `.xls` nhị phân cũ, `.xlsm`, CSV và các định dạng spreadsheet khác.
- OCR cho PDF scan, PDF form/XFA, chữ chuyển thành vector outline hoặc PDF có DRM/password.
- Tự phát hiện ngôn ngữ nguồn; người dùng phải chọn rõ để checkpoint và prompt có tính xác định.
- Dịch sheet name, formula, named range, comment/note, macro, metadata hoặc Markdown code/raw HTML.
- Batch translation, glossary, translation memory và bilingual output.

## Kiến trúc mục tiêu

```text
React UI
  ├─ input + model + source/target language
  ├─ output preview + format capability warnings
  └─ progress/recovery theo document location
            │ typed Tauri DTO
Rust application core
  ├─ LanguageCatalog + TranslationConfig
  ├─ dynamic prompt + protected-content validation
  ├─ DocumentSession (format-neutral orchestration)
  │    ├─ OOXML: DOCX / PPTX / XLSX
  │    ├─ text: Markdown / TXT
  │    └─ PDF: text/layout-aware adapter
  ├─ atomic exporter per storage representation
  └─ checkpoint schema + report
            │ localhost only
          Ollama
```

Không đưa `match` cho từng extension vào job loop. `DocumentSession` sở hữu `kind`, translation units, warnings và representation nội bộ; orchestrator chỉ gọi `open`, `units`, `set_translation` và `export_atomic`. OOXML tiếp tục dùng mutation tối thiểu, còn text/PDF có exporter riêng nhưng chung contract an toàn.

## Các phase

| Phase | Mục tiêu | Đầu ra chính | Phụ thuộc |
| --- | --- | --- | --- |
| 01 | Language pair xuyên suốt hệ thống | Catalog, DTO/config/prompt động, suffix, checkpoint schema | Không |
| 02 | Abstraction đa định dạng + MD/TXT | `DocumentSession`, exporter chung, Markdown/TXT adapters | Phase 01 |
| 03 | XLSX | Adapter OOXML Excel và golden fixtures | Phase 02 |
| 04 | PDF | Feasibility ADR, adapter PDF text-based và render QA | Phase 02 |
| 05 | UI và recovery | Language selects, file filters, capability states, typed DTO | Phase 01–04 |
| 06 | QA, docs và release | Matrix test, security/resource limits, manual compatibility gates | Phase 03–05 |

Chi tiết:

- [Phase 01: Contract đa ngôn ngữ](phase-01-language-contract.md)
- [Phase 02: Lõi đa định dạng, Markdown và TXT](phase-02-document-core-text.md)
- [Phase 03: Hỗ trợ XLSX](phase-03-xlsx.md)
- [Phase 04: Hỗ trợ PDF có text](phase-04-pdf.md)
- [Phase 05: UI, progress và recovery](phase-05-ui-integration.md)
- [Phase 06: QA, tài liệu và phát hành](phase-06-quality-release.md)

## Phụ thuộc và đường găng

`Phase 01 → Phase 02 → (Phase 03 || Phase 04) → Phase 05 → Phase 06`.

Phase 03 và 04 độc lập sau khi `DocumentSession`/export contract ổn định. PDF có gate riêng: nếu spike không đạt độ tin cậy về text mapping, font Unicode, packaging hoặc license, không bật PDF trong production filter; các format khác không bị chặn.

## Quyết định contract

- `LanguageCode` là giá trị BCP 47 từ catalog backend, không nhận free-form prompt text.
- `StartTranslationRequest` thêm `sourceLanguage` và `targetLanguage`; hai trường đi vào `TranslationConfig`, config hash và checkpoint.
- `list_languages` trả `{ code, nativeName, displayName }` để frontend không duy trì catalog riêng.
- `DocumentKind` mở rộng `docx | pptx | xlsx | pdf | markdown | text` và mỗi kind khai báo capabilities/limitations cho UI.
- Output suffix được tạo ở backend bằng mã target đã sanitize; `.markdown` giữ extension gốc.
- Bump `CHECKPOINT_SCHEMA`; checkpoint cũ không resume âm thầm và được hiển thị là không tương thích/có thể discard.

## Chiến lược kiểm thử

- Unit: catalog/language validation, dynamic prompt, same-language rejection, suffix sanitization, config hash và checkpoint migration.
- Contract: mọi adapter chạy cùng bộ test `open → inspect → units → apply → export`, giữ input bất biến và từ chối overwrite.
- Golden: DOCX/PPTX regression; XLSX giữ formula/style/media; Markdown giữ syntax byte-equivalent ngoài text spans; TXT giữ BOM/newline; PDF giữ page count/page size và có translated text.
- Integration: fake translator cho mọi format, Ollama error/retry/cancel/resume và output collision.
- UI: language selects, file filter, capability warning, output preview, recoverable pair và progress label.
- Manual: mở XLSX bằng Excel/LibreOffice; so sánh render PDF theo trang và thử font Việt/Thái/Hàn/Trung/Nhật.

## Rủi ro chính

| Rủi ro | Mức | Giảm thiểu |
| --- | --- | --- |
| PDF không thể thay text an toàn vì font/encoding/content stream | Rất cao | Spike + ADR + fixture gate; chỉ hỗ trợ PDF có text; không bật filter nếu chưa đạt acceptance |
| Text đích dài hơn làm tràn cell/shape/page | Cao | Warning theo expansion ratio, không tự phá geometry; fixture đa script và manual render |
| Shared strings XLSX dùng ở nhiều cell | Cao | Lập reference map, dịch mỗi shared-string record một lần và báo location đại diện |
| Markdown rewrite làm đổi syntax | Cao | Parser có source offsets, thay text spans theo thứ tự ngược; golden byte diff |
| Resume dùng sai cặp ngôn ngữ | Cao | Language pair thuộc config hash + checkpoint schema mới |
| Model không giỏi một language pair | Trung bình | Prompt dùng tên chuẩn, validation cấu trúc; UI không cam kết chất lượng model |

## Tiêu chí thành công

- Người dùng chọn được source/target, request backend và prompt dùng đúng pair, không cho chọn trùng.
- Output suffix đúng target code và không overwrite input/output có sẵn.
- DOCX/PPTX hiện tại không regression.
- XLSX output mở không repair, formula/style/media không đổi và text cell được dịch.
- Markdown/TXT chỉ đổi nội dung được phép dịch; code, URL, syntax, encoding/newline được giữ.
- PDF được bật chỉ khi fixture text-based đạt page/font/render gates; PDF scan nhận lỗi rõ ràng yêu cầu OCR chưa hỗ trợ.
- Cancel/resume giữ đúng format + language pair và không nhận checkpoint schema cũ như tương thích.
- `npm run check` pass và docs/known limitations phản ánh trung thực phạm vi.

## Definition of Done cho mỗi phase

- Code format/lint/test pass cho phần thay đổi.
- DTO Rust/TypeScript và docs được cập nhật cùng lúc.
- Có fixture cho happy path, corrupt/unsupported input và giới hạn tài nguyên.
- Không giảm các invariant offline-only, input immutable, atomic output và redacted logs.
