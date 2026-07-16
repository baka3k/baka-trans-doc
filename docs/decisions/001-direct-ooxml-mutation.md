# ADR-001: Chỉnh trực tiếp gói OOXML

- **Status:** accepted
- **Date:** 2026-07-16

## Context

Việc dựng lại DOCX/PPTX từ mô hình trung gian thường làm mất relationship, style, media hoặc layout không được thư viện biểu diễn đầy đủ.

## Decision

Đọc tài liệu như một gói ZIP có giới hạn tài nguyên, chỉ thay nội dung node `w:t` và `a:t` đã được mapping, rồi đóng gói và kiểm tra lại trước khi atomic rename. Part không đổi giữ nguyên dữ liệu giải nén byte-for-byte.

## Consequences

Giữ được phần lớn cấu trúc Office mà không cần Office Interop. Parser phải xử lý XML/ZIP thận trọng và cần fixture cho các biến thể OOXML.

## Alternatives Considered

- Dựng lại tài liệu bằng thư viện document object model: dễ dùng nhưng rủi ro mất đặc tính không được hỗ trợ.
- Office COM: fidelity tốt trên Windows nhưng phá vỡ mục tiêu cross-platform và yêu cầu Office được cài.

