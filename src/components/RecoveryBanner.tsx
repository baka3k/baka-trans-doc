import type { RecoverableJob } from "../types/job";

interface Props {
  job: RecoverableJob;
  onResume: () => void;
  onDiscard: () => void;
}

export function RecoveryBanner({ job, onResume, onDiscard }: Props) {
  return (
    <aside className="recovery-banner" aria-label="Khôi phục công việc">
      <div>
        <span className="eyebrow">{job.compatible ? "CÓ THỂ KHÔI PHỤC" : "CHECKPOINT KHÔNG TƯƠNG THÍCH"}</span>
        <strong>{job.inputPath.split(/[\\/]/).pop()}</strong>
        <small>{job.sourceLanguage} → {job.targetLanguage} · {job.format.toUpperCase()} · {job.completedUnits} mục · model {job.model}</small>
      </div>
      <div className="inline-actions">
        {job.compatible && <button className="button compact" type="button" onClick={onResume}>Tiếp tục</button>}
        <button className="text-button" type="button" onClick={onDiscard}>Bỏ checkpoint</button>
      </div>
    </aside>
  );
}
