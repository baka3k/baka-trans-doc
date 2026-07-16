# ADR-003: Typed progress event và checkpoint theo unit

- **Status:** accepted
- **Date:** 2026-07-16

## Context

Một tài liệu lớn có thể chạy lâu; UI không được suy diễn trạng thái từ chuỗi log và kết quả đã validate không nên mất khi app đóng.

## Decision

Backend phát event `job-progress` có job ID, phase, trạng thái, phần trăm, ETA và vị trí hiện tại. Sau mỗi unit, manifest JSON được ghi atomically trong app-data với input/config hash và kết quả đã validate. Resume chỉ chạy khi schema, adapter, input và config còn tương thích.

## Consequences

UI bỏ event trễ từ job khác và progress luôn đơn điệu. Checkpoint có thể chứa nội dung tài liệu, vì vậy không được log hoặc gửi ra ngoài và được xóa sau khi job thành công.

## Alternatives Considered

Chỉ lưu phần trăm không đủ để resume an toàn; lưu output dở dang trực tiếp có thể khiến người dùng nhầm file chưa hoàn chỉnh là kết quả.

