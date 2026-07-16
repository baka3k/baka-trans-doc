# ADR-002: Tách domain core khỏi Tauri

- **Status:** accepted
- **Date:** 2026-07-16

## Context

Document parsing, validation và job recovery cần test được mà không mở desktop window.

## Decision

Các module `document`, `translation`, `job` và `storage` không phụ thuộc React. Tauri commands chỉ chuyển đổi input, quản lý cancellation token và phát typed event.

## Consequences

Core có thể unit test độc lập và thay fake translator. Command surface nhỏ, nhưng các DTO ở boundary phải giữ ổn định và dùng camelCase cho frontend.

## Alternatives Considered

Đặt toàn bộ workflow trong command giúp scaffold nhanh hơn nhưng làm cancellation, test và recovery khó kiểm soát.

