import type { RecoverableJob } from "../types/job";

interface Props {
  job: RecoverableJob;
  onResume: () => void;
  onDiscard: () => void;
}

export function RecoveryBanner({ job, onResume, onDiscard }: Props) {
  return (
    <aside className="recovery-banner" aria-label="Recover translation job">
      <div>
        <span className="eyebrow">{job.compatible ? "RECOVERY AVAILABLE" : "INCOMPATIBLE CHECKPOINT"}</span>
        <strong>{job.inputPath.split(/[\\/]/).pop()}</strong>
        <small>{job.sourceLanguage} → {job.targetLanguage} · {job.format.toUpperCase()} · {job.completedUnits} items · model {job.model}</small>
      </div>
      <div className="inline-actions">
        {job.compatible && <button className="button compact" type="button" onClick={onResume}>Resume</button>}
        <button className="text-button" type="button" onClick={onDiscard}>Discard checkpoint</button>
      </div>
    </aside>
  );
}
