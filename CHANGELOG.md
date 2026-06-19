# Changelog

All notable changes to the ANVAYA Platform will be documented in this file.

## [0.1.0] — 2026-06-19

### Added

- **ANVAYA Studio**: Visual circuit editor with drag‑and‑drop gate placement using React Flow.
- **ANVAYA Core**: Quantum circuit representation, state vector simulator, circuit optimizer, OpenQASM 3.0 parser/exporter, compiled to WebAssembly.
- **ANVAYA Pulse**: Pulse scheduler library generating timed pulse sequences from circuits.
- **Studio features**:
  - Real‑time probability histogram after simulation.
  - 3D Bloch sphere for single‑qubit states.
  - Pulse timeline Gantt chart via Wasm.
  - Circuit optimization with gate cancellation and rotation merging.
  - OpenQASM import/export.
  - Local noisy simulator backend with HTTP API.
- **CI pipeline**: GitHub Actions with Rust lint/build/test, TypeScript lint, and Playwright end‑to‑end tests.
- **Developer tooling**: Nx monorepo, pnpm workspaces, Rust workspace, `wasm-pack` integration.

[0.1.0]: https://github.com/anvaya/anvaya-platform/releases/tag/v0.1.0
