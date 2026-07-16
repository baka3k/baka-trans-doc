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
  listModels,
  listRecoverableJobs,
  onJobProgress,
  resumeTranslation,
  showOutput,
  startTranslation,
} from "../lib/tauri";
import { acceptProgress, type JobViewState } from "../store/job";
import type { InputInspection, ModelInfo } from "../types/document";
import type { RecoverableJob } from "../types/job";

const DEFAULT_ENDPOINT = "http://localhost:11434";

function readableError(error: unknown): string {
  if (typeof error === "string") return error;
  if (error && typeof error === "object" && "message" in error) return String(error.message);
  return "Đã xảy ra lỗi không xác định";
}

export function App() {
  const [endpoint, setEndpoint] = useState(DEFAULT_ENDPOINT);
  const [inspection, setInspection] = useState<InputInspection>();
  const [outputFolder, setOutputFolder] = useState("");
  const [models, setModels] = useState<ModelInfo[]>([]);
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
      setError("Mở ứng dụng bằng Tauri để kết nối Ollama và chọn file cục bộ.");
      return;
    }
    void refreshModels();
    void listRecoverableJobs().then(setRecoverable).catch(() => setRecoverable([]));
    let cleanup: (() => void) | undefined;
    void onJobProgress((progress) => {
      setJobState((state) => acceptProgress(state, progress));
    }).then((unlisten) => { cleanup = unlisten; });
    return () => cleanup?.();
  }, [refreshModels]);

  const selectInput = async () => {
    const path = await chooseInput();
    if (!path) return;
    setError("");
    try {
      setInspection(await inspectInput(path));
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
    inspection && outputFolder && model && ollamaState === "ready" &&
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
          <p className="brand-subtitle">Nhật → Việt · Riêng tư · Ngoại tuyến</p>
        </div>
        <div className={`status-pill ${ollamaState}`}>
          <i /> Ollama {ollamaState === "ready" ? "đã kết nối" : ollamaState === "loading" ? "đang kiểm tra" : "chưa sẵn sàng"}
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
            <div><h1>Chuẩn bị bản dịch</h1><p>Chọn tài liệu và cấu hình xử lý cục bộ.</p></div>
          </div>

          <label className="field-label">Tài liệu nguồn</label>
          <button type="button" className={`file-drop ${inspection ? "selected" : ""}`} onClick={() => void selectInput()}>
            <span className="file-icon">{inspection?.kind === "pptx" ? "P" : "W"}</span>
            <span>
              <strong>{inspection?.fileName ?? "Chọn file DOCX hoặc PPTX"}</strong>
              <small>{inspection ? `${inspection.unitCount.toLocaleString("vi-VN")} mục · ${inspection.characterCount.toLocaleString("vi-VN")} ký tự` : "Không tải lên cloud — file luôn ở trên máy"}</small>
            </span>
            <b>{inspection ? "Đổi file" : "Duyệt"}</b>
          </button>

          <div className="language-row">
            <div><label>Nguồn</label><strong><span>日</span> Tiếng Nhật</strong></div>
            <i>→</i>
            <div><label>Đích</label><strong><span>V</span> Tiếng Việt</strong></div>
          </div>

          <div className="field-grid">
            <label>
              <span className="field-label">Model Ollama</span>
              <select value={model} onChange={(event) => setModel(event.target.value)} disabled={ollamaState !== "ready"}>
                <option value="">{ollamaState === "loading" ? "Đang tải model…" : "Chọn model"}</option>
                {models.map((item) => <option key={item.name} value={item.name}>{item.name}</option>)}
              </select>
            </label>
            <label>
              <span className="field-label">Ollama endpoint</span>
              <div className="input-action"><input value={endpoint} onChange={(event) => setEndpoint(event.target.value)} /><button type="button" onClick={() => void refreshModels()}>Kiểm tra</button></div>
            </label>
          </div>

          <label className="field-label">Thư mục kết quả</label>
          <button type="button" className="path-picker" onClick={() => void selectOutput()}>
            <span>{outputFolder || "Chọn thư mục lưu file dịch"}</span><b>Duyệt</b>
          </button>
          {inspection && outputFolder && <p className="output-preview">Sẽ tạo: <strong>{inspection.outputName}</strong></p>}
          {error && <div className="error-banner" role="alert">{error}</div>}

          <button type="button" className="button primary" disabled={!canStart} onClick={() => void begin()}>
            Bắt đầu dịch <span>→</span>
          </button>
        </section>

        <aside className="status-panel">
          <div className="section-intro compact-intro">
            <span className="step-number">02</span>
            <div><h1>Tiến độ</h1><p>Theo dõi từng paragraph hoặc slide.</p></div>
          </div>
          <ProgressPanel
            progress={progress}
            onCancel={() => progress && void cancelTranslation(progress.jobId)}
          />
          {progress?.outputPath && (
            <button className="button result" type="button" onClick={() => void showOutput(progress.outputPath!)}>
              Mở vị trí file kết quả
            </button>
          )}
          <div className="activity-log">
            <div className="log-heading"><h3>Nhật ký</h3><span>{jobState.logs.length} sự kiện</span></div>
            {jobState.logs.length === 0 ? <p>Chưa có hoạt động.</p> : jobState.logs.slice(-5).reverse().map((entry, index) => (
              <div className="log-line" key={`${entry.phase}-${entry.current}-${index}`}><i className={entry.warning ? "warn" : ""}/><span>{entry.message}<small>{entry.currentItem}</small></span></div>
            ))}
          </div>
          {recentWarnings.length > 0 && <p className="privacy-note">Có {recentWarnings.length} cảnh báo gần đây. Nội dung tài liệu không được ghi vào log.</p>}
        </aside>
      </div>
      <footer><span>Offline by design</span><span>DOCX · PPTX</span><span>Không telemetry</span></footer>
    </main>
  );
}

