# Phase 01: Contract đa ngôn ngữ

## Bối cảnh

Ngôn ngữ hiện bị hard-code trong `src-tauri/src/translation/prompt.rs`, `src/app/App.tsx`, README và hậu tố `_vi`. `StartTranslationRequest` và `TranslationConfig` chưa chứa language pair nên checkpoint có thể resume mà không biết cặp ngôn ngữ.

## Yêu cầu

- Backend là source of truth cho catalog ngôn ngữ.
- Source/target bắt buộc, thuộc catalog và khác nhau.
- Prompt động không nhận tên ngôn ngữ tùy ý từ UI.
- Output suffix và recovery metadata phản ánh target/source.
- Checkpoint cũ bị đánh dấu không tương thích thay vì deserialize/resume sai.

## Kiến trúc

Thêm `translation/language.rs` với `LanguageCode`/`LanguageInfo`, catalog tĩnh và lookup. DTO dùng code BCP 47; prompt nhận `LanguageInfo` đã lookup để sinh system instruction. `TranslationConfig` chứa pair để `config_hash` hiện tại tự bao phủ thay đổi. Bump checkpoint schema và thêm pair vào `RecoverableJob`.

## File liên quan

- `src-tauri/src/translation/{language.rs,prompt.rs,retry.rs,mod.rs}`
- `src-tauri/src/{config.rs,commands/mod.rs}`
- `src-tauri/src/job/{mod.rs,checkpoint.rs,orchestrator.rs}`
- `src-tauri/src/document/mod.rs`
- `src/types/{document.ts,job.ts}`
- `src/lib/tauri.ts`
- `docs/specs.md`

## Các bước thực hiện

1. Định nghĩa catalog ban đầu và command `list_languages`; serialize camelCase nhất quán.
2. Thêm `source_language`/`target_language` vào request/config và validation: required, supported, distinct.
3. Đổi `build_prompt(text, repair)` thành API nhận language pair; giữ repair rule và marker invariant.
4. Đưa target code vào hàm tạo output name/path dùng chung cho inspection preview và job export; loại bỏ `_vi` hard-code.
5. Bump `CHECKPOINT_SCHEMA`, cập nhật recoverable DTO và thông báo checkpoint cũ.
6. Cập nhật fake translator/unit tests để chứng minh prompt chứa đúng pair và retry vẫn giữ marker.
7. Ghi ADR cho language identifier/catalog và cập nhật specs.

## Todo

- [ ] UI không thể truyền free-form language name vào prompt.
- [ ] Pair trùng hoặc code lạ bị chặn trước khi spawn job.
- [ ] Output name dùng target code an toàn trên Windows.
- [ ] Config hash thay đổi khi đổi source hoặc target.
- [ ] Checkpoint schema cũ không crash khi list/discard.

## Rủi ro

- Tên ngôn ngữ có thể thành prompt injection nếu lấy trực tiếp từ UI; chỉ dùng catalog backend.
- BCP 47 có dấu gạch ngang/case; chuẩn hóa lookup nhưng giữ một canonical code duy nhất khi serialize.

## Tiêu chí thành công

- Test cover ít nhất `ja→vi`, `vi→en`, `zh-Hans→th`, same-language và unknown-code.
- Prompt, output name, config hash và recovery view cùng phản ánh một language pair.

