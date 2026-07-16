export type JobStatus =
  | "queued"
  | "running"
  | "cancelling"
  | "completed"
  | "completed_with_warnings"
  | "failed"
  | "cancelled";

export interface JobProgress {
  jobId: string;
  status: JobStatus;
  phase: string;
  current: number;
  total: number;
  percent: number;
  etaSeconds?: number;
  currentItem?: string;
  warning?: string;
  message: string;
  outputPath?: string;
}

export interface StartTranslationRequest {
  inputPath: string;
  outputFolder: string;
  endpoint: string;
  model: string;
  sourceLanguage: string;
  targetLanguage: string;
  chunkChars?: number;
}

export interface RecoverableJob {
  jobId: string;
  status: JobStatus;
  inputPath: string;
  model: string;
  sourceLanguage: string;
  targetLanguage: string;
  format: string;
  compatible: boolean;
  completedUnits: number;
  warningCount: number;
  updatedAt: string;
}
