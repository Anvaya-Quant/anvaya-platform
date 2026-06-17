# ANVAYA Platform – Agent Instructions

## Project Mission
Build the Phase 1 MVP of ANVAYA, a hardware-agnostic quantum computing platform.  
Phase 1 (The Wedge) includes three products:
- **ANVAYA Studio** – visual quantum circuit IDE (Next.js, TypeScript)
- **ANVAYA Core** – Rust/Wasm quantum simulator and circuit compiler
- **ANVAYA Pulse** – pulse schedule generator (simulated for MVP)

These three layers form a complete loop: design a circuit in Studio → optimize/simulate in Core → generate pulses in Pulse → run on a simulated quantum backend → results back to Studio.

## Repository Layout
We use a **hybrid** approach:
- `anvaya-platform` (monorepo) – contains the three Phase 1 products and any future core platform components.
- Vertical solutions (Chem, Fin, etc.) will live in separate repos later; for now they do not exist.

The monorepo structure:

anvaya-platform/
├── apps/
│ └── studio/ # Next.js app
├── packages/
│ └── anvaya-core-js/ # npm package wrapping the Wasm output
├── rust/
│ ├── Cargo.toml # workspace manifest
│ ├── anvaya-core/ # main simulation library (lib crate)
│ ├── anvaya-core-wasm/ # wasm-bindgen crate, targets wasm32
│ ├── anvaya-pulse/ # pulse scheduling library (lib crate)
│ └── anvaya-simulator/ # (optional) CLI/local runner for integration tests
├── shared/
│ └── types/ # Generated TypeScript types (output of wasm-pack)
├── scripts/
│ └── build-wasm.sh
├── nx.json
├── package.json # root workspace for npm packages
├── turbo.json # (not used, but may be present; we use Nx)
└── agents.md # this file

text

## Technology Stack
- **Monorepo orchestration**: Nx (task graph, caching, cross-language builds)
- **Rust workspace**: Cargo (manages internal Rust dependencies)
- **Frontend**: Next.js (App Router), TypeScript, Tailwind CSS, React Flow (for circuit canvas)
- **State management in Studio**: Zustand (lightweight)
- **Visualization**: Three.js (Bloch sphere), D3.js or custom Canvas (probability histograms)
- **Wasm generation**: wasm-bindgen, wasm-pack
- **Testing**: Jest (JS), Rust's built-in test framework, Playwright for E2E later
- **Formatting/Linting**: Prettier, ESLint, rustfmt, clippy
- **Git hooks**: husky + lint-staged (optional, we can add later)

## Git Workflow (Strict)
- **main** – production-ready code, never committed to directly.
- **develop** – integration branch, all features merge here.
- **Feature branches** – created from `develop`, named `feature/<short-description>`.  
  Example: `feature/circuit-canvas`, `feature/bloch-sphere`.
- **Workflow**:
  1. Pull latest `develop`.
  2. Create feature branch.
  3. Commit work, push, open PR to `develop`.
  4. After review & CI, merge into `develop`.
  5. **Only when a release is ready**, merge `develop` into `main` with a version tag (e.g., `v0.1.0`).
- **Never** work directly on `develop` or `main`.
- Always do the conventional and granular commits at all cost.
- All sessions assume the agent is working on a feature branch that will eventually be merged into `develop`.

## Code Quality & Testing
- All Rust code must be formatted (`cargo fmt`), linted (`cargo clippy`), and pass `cargo test`.
- All TypeScript code must pass `npx eslint` and `npm run test` (if tests exist).
- New functionality must have corresponding tests (unit tests for Core/Pulse, component tests for Studio).
- Commit messages should be clear and concise (e.g., "feat: add basic state vector simulator").

## Nx Tasks and Dependencies
- The `anvaya-core-js` package depends on the Rust WASM build.  
  The Nx pipeline ensures `studio` depends on `anvaya-core-js`, and `anvaya-core-js` depends on a custom target that runs `wasm-pack build`.
- **Important**: The agent should not manually tweak Nx configuration unless instructed; we will handle the `nx.json` and project configuration in the initial sessions. After that, the agent works inside the projects.

## Session Behaviour
- At the start of every session, the agent receives the current state of the repository (already checked out on a feature branch).
- The agent reads this `agents.md` file.
- The agent is given a specific, step-by-step task (the session brief). The task may include file paths, expected behaviour, and acceptance criteria.
- The agent must:
  - Understand the requirement.
  - Implement the changes, adding or modifying files as needed.
  - Write tests if the task requires new logic.
  - Ensure existing tests still pass.
  - If the task is purely a code change, output only the changed files and a summary of what was done.
- The agent must **never** merge or push to `main` or `develop` directly. It only works on the designated feature branch. The user will handle merging.
- The agent must output all file contents that are new or modified, so the user can copy them directly.

## Building and Running
- To build the Rust workspace: `cd rust && cargo build`
- To build the WASM package: `cd rust/anvaya-core-wasm && wasm-pack build --target web --out-dir ../../packages/anvaya-core-js/pkg`
- To run the Studio dev server: `cd apps/studio && npm run dev`
- To run all tests: `cargo test` from `rust/`, and `npm test` from `apps/studio`.

That’s it. Follow these rules in every session.