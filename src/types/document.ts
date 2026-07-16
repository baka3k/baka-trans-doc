export type DocumentKind = "docx" | "pptx";

export interface InputInspection {
  path: string;
  fileName: string;
  kind: DocumentKind;
  unitCount: number;
  characterCount: number;
  outputName: string;
  warnings: string[];
}

export interface ModelInfo {
  name: string;
  size?: number;
  modified_at?: string;
}

