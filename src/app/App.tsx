import { useCallback, useEffect, useMemo, useState } from "react";
import { ProgressPanel } from "../components/ProgressPanel";
import { RecoveryBanner } from "../components/RecoveryBanner";
import {
  cancelTranslation,
  chooseInput,
  chooseOutputFolder,
  discardJob,
  inspectInput,
  isDesktop,
  listLanguages,
  listModels,
  listRecoverableJobs,
  onJobProgress,
  previewOutputName,
  resumeTranslation,
  showOutput,
  startTranslation,
} from "../lib/tauri";
import { acceptProgress, type JobViewState } from "../store/job";
import type { DocumentKind, InputInspection, LanguageInfo, ModelInfo } from "../types/document";
import type { RecoverableJob } from "../types/job";

const DEFAULT_ENDPOINT = "http://localhost:11434";
const FILE_ICONS: Record<DocumentKind, string> = { docx: "W", pptx: "P", xlsx: "X", pdf: "PDF", markdown: "MD", text: "TXT" };

function readableError(error: unknown): string {
  if (typeof error === "string") return error;
  if (error && typeof error === "object" && "message" in error) return String(error.message);
  return "An unknown error occurred";
}

export function App() {
  const [endpoint, setEndpoint] = useState(DEFAULT_ENDPOINT);
  const [inspection, setInspection] = useState<InputInspection>();
  const [outputFolder, setOutputFolder] = useState("");
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [languages, setLanguages] = useState<LanguageInfo[]>([]);
  const [sourceLanguage, setSourceLanguage] = useState("ja");
  const [targetLanguage, setTargetLanguage] = useState("vi");
  const [model, setModel] = useState("");
  const [ollamaState, setOllamaState] = useState<"loading" | "ready" | "error">("loading");
  const [error, setError] = useState("");
  const [jobState, setJobState] = useState<JobViewState>({ logs: [] });
  const [recoverable, setRecoverable] = useState<RecoverableJob[]>([]);

  const refreshModels = useCallback(async () => {
    setOllamaState("loading");
    setError("");
    try {
      const result = await listModels(endpoint);
      setModels(result);
      setModel((current) => current || result[0]?.name || "");
      setOllamaState("ready");
    } catch (cause) {
      setModels([]);
      setOllamaState("error");
      setError(readableError(cause));
    }
  }, [endpoint]);

  useEffect(() => {
    if (!isDesktop()) {
      setOllamaState("error");
      setError("Open the Tauri desktop app to connect to Ollama and select local files.");
      return;
    }
    void refreshModels();
    void listLanguages().then((items) => {
      setLanguages(items);
      setSourceLanguage((current) => items.some((item) => item.code === current) ? current : items[0]?.code ?? "");
      setTargetLanguage((current) => items.some((item) => item.code === current) ? current : items[1]?.code ?? "");
    }).catch((cause) => setError(readableError(cause)));
    void listRecoverableJobs().then(setRecoverable).catch(() => setRecoverable([]));
    let cleanup: (() => void) | undefined;
    void onJobProgress((progress) => {
      setJobState((state) => acceptProgress(state, progress));
    }).then((unlisten) => { cleanup = unlisten; });
    return () => cleanup?.();
  }, [refreshModels]);

  useEffect(() => {
    if (!inspection || !targetLanguage) return;
    void previewOutputName(inspection.path, targetLanguage)
      .then((outputName) => setInspection((current) => current?.path === inspection.path ? { ...current, outputName } : current))
      .catch((cause) => setError(readableError(cause)));
  }, [inspection?.path, targetLanguage]);

  const selectInput = async () => {
    const path = await chooseInput();
    if (!path) return;
    setError("");
    try {
      setInspection(await inspectInput(path, targetLanguage));
    } catch (cause) {
      setInspection(undefined);
      setError(readableError(cause));
    }
  };

  const selectOutput = async () => {
    const folder = await chooseOutputFolder();
    if (folder) setOutputFolder(folder);
  };

  const canStart = Boolean(
    inspection && inspection.capabilities.canTranslate && outputFolder && model && languages.length > 0 &&
    sourceLanguage && targetLanguage && sourceLanguage !== targetLanguage && ollamaState === "ready" &&
    !["running", "queued"].includes(jobState.progress?.status ?? ""),
  );

  const begin = async () => {
    if (!inspection || !canStart) return;
    setError("");
    setJobState({ logs: [] });
    try {
      const jobId = await startTranslation({
        inputPath: inspection.path,
        outputFolder,
        endpoint,
        model,
        sourceLanguage,
        targetLanguage,
        chunkChars: 1800,
      });
      setJobState({ activeJobId: jobId, logs: [] });
    } catch (cause) {
      setError(readableError(cause));
    }
  };

  const resume = async (job: RecoverableJob) => {
    setError("");
    try {
      const jobId = await resumeTranslation(job.jobId);
      setJobState({ activeJobId: jobId, logs: [] });
      setRecoverable((items) => items.filter((item) => item.jobId !== jobId));
    } catch (cause) {
      setError(readableError(cause));
    }
  };

  const recentWarnings = useMemo(
    () => jobState.logs.filter((entry) => entry.warning).slice(-5),
    [jobState.logs],
  );
  const progress = jobState.progress;

  return (
    <main className="app-shell">
      <header className="topbar">
        <div className="brand-mark">文<span>V</span></div>
        <div>
          <p className="brand-title">LOCAL DOCUMENT TRANSLATOR</p>
          <p className="brand-subtitle">Multilingual · Private · Offline</p>
        </div>
        <div className={`status-pill ${ollamaState}`}>
          <i /> Ollama {ollamaState === "ready" ? "connected" : ollamaState === "loading" ? "checking" : "not ready"}
        </div>
      </header>

      {recoverable[0] && (
        <RecoveryBanner
          job={recoverable[0]}
          onResume={() => void resume(recoverable[0])}
          onDiscard={() => {
            const job = recoverable[0];
            void discardJob(job.jobId).then(() => setRecoverable((items) => items.slice(1)));
          }}
        />
      )}

      <div className="workspace">
        <section className="form-panel">
          <div className="section-intro">
            <span className="step-number">01</span>
            <div><h1>Prepare translation</h1><p>Choose a document and configure local processing.</p></div>
          </div>

          <label className="field-label">Source document</label>
          <button type="button" className={`file-drop ${inspection ? "selected" : ""}`} onClick={() => void selectInput()}>
            <span className="file-icon">{inspection ? FILE_ICONS[inspection.kind] : "DOC"}</span>
            <span>
              <strong>{inspection?.fileName ?? "Choose a DOCX, PPTX, XLSX, Markdown, or TXT file"}</strong>
              <small>{inspection ? `${inspection.unitCount.toLocaleString("en-US")} items · ${inspection.characterCount.toLocaleString("en-US")} characters` : "Nothing is uploaded — the file stays on this device"}</small>
            </span>
            <b>{inspection ? "Change" : "Browse"}</b>
          </button>

          <div className="language-row">
            <label><span>Source</span><select aria-label="Source language" value={sourceLanguage} onChange={(event) => setSourceLanguage(event.target.value)}>
              {languages.map((language) => <option key={language.code} value={language.code}>{language.nativeName} · {language.displayName}</option>)}
            </select></label>
            <i>→</i>
            <label><span>Target</span><select aria-label="Target language" value={targetLanguage} onChange={(event) => setTargetLanguage(event.target.value)}>
              {languages.map((language) => <option key={language.code} value={language.code}>{language.nativeName} · {language.displayName}</option>)}
            </select></label>
          </div>
          {sourceLanguage === targetLanguage && <div className="error-banner" role="alert">Source and target languages must be different.</div>}

          <div className="field-grid">
            <label>
              <span className="field-label">Ollama model</span>
              <select value={model} onChange={(event) => setModel(event.target.value)} disabled={ollamaState !== "ready"}>
                <option value="">{ollamaState === "loading" ? "Loading models…" : "Select a model"}</option>
                {models.map((item) => <option key={item.name} value={item.name}>{item.name}</option>)}
              </select>
            </label>
            <label>
              <span className="field-label">Ollama endpoint</span>
              <div className="input-action"><input value={endpoint} onChange={(event) => setEndpoint(event.target.value)} /><button type="button" onClick={() => void refreshModels()}>Check</button></div>
            </label>
          </div>

          <label className="field-label">Output folder</label>
          <button type="button" className="path-picker" onClick={() => void selectOutput()}>
            <span>{outputFolder || "Choose where to save translated files"}</span><b>Browse</b>
          </button>
          {inspection && outputFolder && <p className="output-preview">Will create: <strong>{inspection.outputName}</strong></p>}
          {inspection && [...inspection.capabilities.limitations, ...inspection.warnings].map((warning) => <div className="capability-note" key={warning}>{warning}</div>)}
          {error && <div className="error-banner" role="alert">{error}</div>}

          <button type="button" className="button primary" disabled={!canStart} onClick={() => void begin()}>
            Start translation <span>→</span>
          </button>
        </section>

        <aside className="status-panel">
          <div className="section-intro compact-intro">
            <span className="step-number">02</span>
            <div><h1>Progress</h1><p>Track each item using its format-specific location.</p></div>
          </div>
          <ProgressPanel
            progress={progress}
            onCancel={() => progress && void cancelTranslation(progress.jobId)}
          />
          {progress?.outputPath && (
            <button className="button result" type="button" onClick={() => void showOutput(progress.outputPath!)}>
              Show output in folder
            </button>
          )}
          <div className="activity-log">
            <div className="log-heading"><h3>Activity</h3><span>{jobState.logs.length} {jobState.logs.length === 1 ? "event" : "events"}</span></div>
            {jobState.logs.length === 0 ? <p>No activity yet.</p> : jobState.logs.slice(-5).reverse().map((entry, index) => (
              <div className="log-line" key={`${entry.phase}-${entry.current}-${index}`}><i className={entry.warning ? "warn" : ""}/><span>{entry.message}<small>{entry.currentItem}</small></span></div>
            ))}
          </div>
          {recentWarnings.length > 0 && <p className="privacy-note">{recentWarnings.length} recent {recentWarnings.length === 1 ? "warning" : "warnings"}. Document content is never written to logs.</p>}
        </aside>
      </div>
      <footer><span>Offline by design</span><span>DOCX · PPTX · XLSX · MD · TXT</span><span>No telemetry</span></footer>
    </main>
  );
}
