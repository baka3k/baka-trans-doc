import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import type { InputInspection, ModelInfo } from "../types/document";
import type {
  JobProgress,
  RecoverableJob,
  StartTranslationRequest,
} from "../types/job";

export const isDesktop = () => "__TAURI_INTERNALS__" in window;

export async function chooseInput(): Promise<string | null> {
  if (!isDesktop()) return null;
  const selected = await open({
    multiple: false,
    directory: false,
    filters: [{ name: "Office documents", extensions: ["docx", "pptx"] }],
  });
  return selected;
}

export async function chooseOutputFolder(): Promise<string | null> {
  if (!isDesktop()) return null;
  const selected = await open({ multiple: false, directory: true });
  return selected;
}

export const inspectInput = (path: string) =>
  invoke<InputInspection>("inspect_input", { path });

export const listModels = (endpoint: string) =>
  invoke<ModelInfo[]>("list_models", { endpoint });

export const startTranslation = (request: StartTranslationRequest) =>
  invoke<string>("start_translation", { request });

export const cancelTranslation = (jobId: string) =>
  invoke<void>("cancel_translation", { jobId });

export const listRecoverableJobs = () =>
  invoke<RecoverableJob[]>("list_recoverable_jobs");

export const resumeTranslation = (jobId: string) =>
  invoke<string>("resume_translation", { jobId });

export const discardJob = (jobId: string) =>
  invoke<void>("discard_job", { jobId });

export const onJobProgress = (
  handler: (progress: JobProgress) => void,
): Promise<UnlistenFn> =>
  listen<JobProgress>("job-progress", (event) => handler(event.payload));

export async function showOutput(path: string): Promise<void> {
  await revealItemInDir(path);
}

