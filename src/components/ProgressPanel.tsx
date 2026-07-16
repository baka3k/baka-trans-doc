import type { JobProgress } from "../types/job";
import { formatEta } from "../store/job";

interface Props {
  progress?: JobProgress;
  onCancel: () => void;
}

const phaseLabel: Record<string, string> = {
  extracting: "Reading document",
  translating: "Translating",
  exporting: "Validating and exporting",
  completed: "Complete",
  cancelled: "Cancelled",
  failed: "Error",
};

export function ProgressPanel({ progress, onCancel }: Props) {
  if (!progress) {
    return (
      <section className="progress-card idle" aria-label="Progress">
        <span className="eyebrow">READY</span>
        <h2>Documents are processed entirely on this device.</h2>
        <p>Select a file, an Ollama model, and an output folder to begin.</p>
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
        <span>{progress.total ? `${progress.current} / ${progress.total} items` : "Preparing"}</span>
        <span>{formatEta(progress.etaSeconds)}</span>
      </div>
      {progress.warning && <p className="warning-note">{progress.warning}</p>}
      {active && (
        <button type="button" className="button ghost danger" onClick={onCancel}>
          Cancel safely
        </button>
      )}
    </section>
  );
}
