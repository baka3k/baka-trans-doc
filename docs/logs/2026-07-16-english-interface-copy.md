# English Interface Copy — 2026-07-16

## Context

The desktop translator interface needed to use English throughout. The work followed a quick inline `hi-craft` plan, so there is no plan file to link.

## Change

Translated the main workflow, model and file controls, language validation, activity area, and privacy footer to English (`src/app/App.tsx:155`, `src/app/App.tsx:177`, `src/app/App.tsx:224`, `src/app/App.tsx:243`, `src/app/App.tsx:251`). Progress phases, idle guidance, item counts, and safe cancellation now use English (`src/components/ProgressPanel.tsx:10`, `src/components/ProgressPanel.tsx:22`, `src/components/ProgressPanel.tsx:48`, `src/components/ProgressPanel.tsx:54`). Recovery controls and ETA formatting were translated with matching test expectations (`src/components/RecoveryBanner.tsx:11`, `src/components/RecoveryBanner.test.tsx:15`, `src/store/job.ts:26`, `src/store/job.test.ts:36`).

## Impact

Risk level: low. User-facing frontend copy is consistently English, including accessibility labels and singular/plural time and event text; translation behavior and document-processing logic are unchanged. `npm run check` passed with 3 frontend tests and 20 Rust tests.

## Decision

Replace the existing Vietnamese literals directly because the request targets a single English interface and does not require runtime locale switching. Keeping the change within the current components and view-state formatter avoids introducing an internationalization framework without a demonstrated localization requirement.

## References

- commit: `f44c1542a40302056a90ff731e85978839389ebb`
- plan: none — quick inline `hi-craft` plan
