# Repair npm Tauri Dev Startup — 2026-07-16

## Context

Running `npm run tauri` initially failed because the package had no matching npm script. After exposing the CLI, Windows dev startup also failed with `EBUSY` because Vite watched the Cargo output executable while it was locked for compilation.

## Change

Added the `tauri` script that forwards arguments to the installed Tauri CLI (`package.json:7`). Vite now ignores `src-tauri/**`; Tauri/Cargo remains responsible for watching Rust sources (`vite.config.ts:10`).

## Impact

Developers can now run commands such as `npm run tauri dev` without Vite attempting to watch locked Rust build artifacts. Risk: low; frontend hot reload remains enabled and Tauri still reloads Rust changes.

## Decision

Expose the CLI through npm so it uses the project-local dependency. Keep the frontend and Rust watcher responsibilities separate because duplicate watching adds no functionality and is unreliable for locked Windows executables.

## References

- `package.json:7`
- `vite.config.ts:10`
- commit: `70849a4856a1e3233257e23137ae8aeae29a4e4c`
- plan: `plans/260716-2039-local-document-translator/plan.md`
