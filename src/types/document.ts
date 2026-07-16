export type DocumentKind = "docx" | "pptx" | "xlsx" | "pdf" | "markdown" | "text";

export interface LanguageInfo {
  code: string;
  nativeName: string;
  displayName: string;
}

export interface DocumentCapabilities {
  canTranslate: boolean;
  limitations: string[];
}

export interface InputInspection {
  path: string;
  fileName: string;
  kind: DocumentKind;
  unitCount: number;
  characterCount: number;
  outputName: string;
  warnings: string[];
  capabilities: DocumentCapabilities;
}

export interface ModelInfo {
  name: string;
  size?: number;
  modified_at?: string;
}
