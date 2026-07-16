import { describe, expect, it } from "vitest";
import { acceptProgress, formatEta, type JobViewState } from "./job";

describe("job view state", () => {
  it("ignores late events from another job", () => {
    const state: JobViewState = { activeJobId: "new", logs: [] };
    const result = acceptProgress(state, {
      jobId: "old",
      status: "running",
      phase: "translating",
      current: 1,
      total: 2,
      percent: 50,
      message: "old",
    });
    expect(result).toBe(state);
  });

  it("keeps progress monotonic and formats ETA", () => {
    const state: JobViewState = {
      activeJobId: "job",
      logs: [],
      progress: {
        jobId: "job",
        status: "running",
        phase: "translating",
        current: 2,
        total: 4,
        percent: 50,
        message: "half",
      },
    };
    expect(
      acceptProgress(state, { ...state.progress!, percent: 40 }).progress?.percent,
    ).toBe(50);
    expect(formatEta(61)).toBe("About 2 minutes");
  });
});
