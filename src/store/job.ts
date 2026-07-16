import type { JobProgress } from "../types/job";

export interface JobViewState {
  activeJobId?: string;
  progress?: JobProgress;
  logs: JobProgress[];
}

export function acceptProgress(
  state: JobViewState,
  progress: JobProgress,
): JobViewState {
  if (state.activeJobId && progress.jobId !== state.activeJobId) return state;
  const previousPercent = state.progress?.percent ?? 0;
  const normalized = {
    ...progress,
    percent: Math.max(previousPercent, progress.percent),
  };
  return {
    activeJobId: progress.jobId,
    progress: normalized,
    logs: [...state.logs.slice(-99), normalized],
  };
}

export function formatEta(seconds?: number): string {
  if (seconds === undefined) return "Estimating";
  if (seconds < 60) return `About ${seconds} second${seconds === 1 ? "" : "s"}`;
  const minutes = Math.ceil(seconds / 60);
  return `About ${minutes} minute${minutes === 1 ? "" : "s"}`;
}
