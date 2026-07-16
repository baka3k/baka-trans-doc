// @vitest-environment jsdom
import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { describe, expect, it, vi } from "vitest";
import { RecoveryBanner } from "./RecoveryBanner";

describe("RecoveryBanner", () => {
  it("shows the pair and format and prevents incompatible resume", () => {
    render(<RecoveryBanner job={{
      jobId: "old", status: "cancelled", inputPath: "C:\\docs\\book.xlsx", model: "qwen",
      sourceLanguage: "ja", targetLanguage: "th", format: "xlsx", compatible: false,
      completedUnits: 3, warningCount: 0, updatedAt: "2026-07-16T00:00:00Z",
    }} onResume={vi.fn()} onDiscard={vi.fn()} />);
    expect(screen.getByText(/ja → th · XLSX/)).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Resume" })).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Discard checkpoint" })).toBeInTheDocument();
  });
});
