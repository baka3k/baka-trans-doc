# DOCX fixture sources

`minimal-document.xml` là part nguồn của fixture DOCX tổng hợp trong Rust tests. Test đóng gói part này cùng content types và media sentinel thành `.docx` tạm thời, dịch text, rồi xác nhận XML đích đổi trong khi media giữ nguyên byte.

