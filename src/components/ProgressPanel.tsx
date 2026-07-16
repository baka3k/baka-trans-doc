import type { JobProgress } from "../types/job";
import { formatEta } from "../store/job";

interface Props {
  progress?: JobProgress;
  onCancel: () => void;
}

const phaseLabel: Record<string, string> = {
  extracting: "Đang đọc tài liệu",
  translating: "Đang dịch",
  exporting: "Đang kiểm tra và xuất file",
  completed: "Hoàn tất",
  cancelled: "Đã hủy",
  failed: "Có lỗi",
};

export function ProgressPanel({ progress, onCancel }: Props) {
  if (!progress) {
    return (
      <section className="progress-card idle" aria-label="Tiến độ">
        <span className="eyebrow">SẴN SÀNG</span>
        <h2>Tài liệu được xử lý hoàn toàn trên máy này.</h2>
        <p>Chọn file, model Ollama và thư mục đích để bắt đầu.</p>
      </section>
    );
  }
  const active = progress.status === "running" || progress.status === "queued";
  return (
    <section className={`progress-card ${progress.status}`} aria-live="polite">
      <div className="progress-heading">
        <div>
          <span className="eyebrow">{phaseLabel[progress.phase] ?? progress.phase}</span>
          <h2>{progress.currentItem ?? progress.message}</h2>
        </div>
        <strong>{Math.round(progress.percent)}%</strong>
      </div>
      <div
        className="progress-track"
        role="progressbar"
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={Math.round(progress.percent)}
      >
        <span style={{ width: `${progress.percent}%` }} />
      </div>
      <div className="progress-meta">
        <span>{progress.total ? `${progress.current} / ${progress.total} mục` : "Đang chuẩn bị"}</span>
        <span>{formatEta(progress.etaSeconds)}</span>
      </div>
      {progress.warning && <p className="warning-note">{progress.warning}</p>}
      {active && (
        <button type="button" className="button ghost danger" onClick={onCancel}>
          Hủy an toàn
        </button>
      )}
    </section>
  );
}

