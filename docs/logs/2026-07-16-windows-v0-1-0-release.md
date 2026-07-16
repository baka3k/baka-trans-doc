# Windows v0.1.0 Release — 2026-07-16

## Context

The Windows desktop application reached its first public operational release. No plan file exists for this release task; the release used the existing `0.1.0` package version (`package.json:4`, `src-tauri/tauri.conf.json:4`) and enabled Tauri bundling (`src-tauri/tauri.conf.json:28`).

## Change

Published GitHub release `v0.1.0` with x64 MSI and NSIS artifacts. At publication, both tag `v0.1.0` and branch `develop` pointed to metadata-correction commit `27b2cd6f66a56160c2a08e0c3b5bbff971f15e28`, which supplies the Windows bundle description (`src-tauri/tauri.conf.json:38`). `npm ci` reported 0 vulnerabilities, and `npm run check` passed 3 frontend and 20 Rust tests through the project quality gate (`package.json:14`). The packaged executable also passed a launch smoke test.

Artifact SHA-256 values:

- MSI x64: `F6E204AA98F7EB16365C2ABD8FE6CF7A3871CD2EF7ADDC4955DC02F3C6955BBC`
- NSIS x64: `3A8E008DA134DB15FAA16E37A0FDE8ADD309AB737C0A7DCE018A41F964974905`

## Impact

Windows users can install v0.1.0 from GitHub and verify either artifact by checksum. Risk level: **medium**, because the installers are unsigned and Windows may display trust warnings; the release notes explicitly disclose this limitation.

## Decision

Published both MSI and NSIS formats after automated checks and executable smoke validation so users can choose their preferred installer. Shipping unsigned artifacts was accepted for v0.1.0 with an explicit release-note warning, while exact SHA-256 values provide integrity verification.

## References

- release: https://github.com/baka3k/baka-trans-doc/releases/tag/v0.1.0
- version and quality gate: `package.json:4`, `package.json:14`
- Windows bundle metadata: `src-tauri/tauri.conf.json:28`, `src-tauri/tauri.conf.json:38`, `src-tauri/tauri.conf.json:40`
- metadata correction and publication commit: `27b2cd6f66a56160c2a08e0c3b5bbff971f15e28`
