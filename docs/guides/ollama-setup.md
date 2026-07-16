# Thiết lập Ollama

## Prerequisites

- Ollama cài trên cùng máy.
- Một model có khả năng dịch Nhật–Việt, ví dụ `qwen3`, `gemma3` hoặc `mistral`.

## Steps

1. Cài Ollama từ nguồn chính thức của hệ điều hành.
2. Tải model, ví dụ `ollama pull qwen3`.
3. Xác nhận `ollama list` hiển thị model và service lắng nghe tại `http://localhost:11434`.
4. Mở ứng dụng, giữ endpoint mặc định và bấm **Kiểm tra**.

## Troubleshooting

- “Ollama chưa sẵn sàng”: chạy `ollama serve`, kiểm tra firewall loopback và thử lại.
- “Model unavailable”: tải đúng tên/tag model rồi refresh danh sách.
- Timeout: model có thể chưa được nạp hoặc thiếu RAM/VRAM. Thử model nhỏ hơn.
- Marker bị model thay đổi: app tự retry bằng prompt chặt hơn; unit vẫn lỗi sẽ được giữ nguyên và báo warning.

## FAQ

Ứng dụng không tải model và không gửi nội dung ra Internet. Trong MVP, endpoint phải là `localhost` hoặc `127.0.0.1`.

