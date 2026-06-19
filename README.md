# ANVAYA Platform

The integrated quantum operating system — from a visual circuit IDE and bare-metal pulse orchestration to cloud QPU routing and vertical industry solutions.

**Phase 1 (The Wedge) — v0.1.0**

- **ANVAYA Studio** — drag‑and‑drop quantum circuit editor with Bloch sphere and probability visualisations
- **ANVAYA Core** — high‑performance Rust/Wasm quantum simulator and compiler (up to 20+ qubits locally)
- **ANVAYA Pulse** — pulse schedule generation for hardware‑level execution (simulated for MVP)

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) ≥ 20
- [pnpm](https://pnpm.io/) ≥ 9
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Setup

```bash
git clone https://github.com/anvaya/anvaya-platform.git
cd anvaya-platform
pnpm install
cargo build --manifest-path rust/Cargo.toml
npx nx build @anvaya/core
npx nx build @anvaya/pulse
```

Run Studio (development)

```bash
cd apps/studio
pnpm dev
```

Open http://localhost:3000.

Run the Simulator Backend (optional)

```bash
cargo run -p anvaya-simulator -- serve
```

Studio's "Run on Simulator" button will POST circuits to this server.

## Repository Structure

```
anvaya-platform/
├── apps/
│   └── studio/                  # Next.js frontend (TypeScript, React Flow, Three.js)
├── packages/
│   ├── anvaya-core-js/          # npm wrapper for Rust core WASM
│   └── anvaya-pulse-js/         # npm wrapper for Rust pulse WASM
├── rust/
│   ├── anvaya-core/             # quantum circuit & simulation library
│   ├── anvaya-core-wasm/        # wasm-bindgen exports for core
│   ├── anvaya-pulse/            # pulse scheduler library
│   ├── anvaya-pulse-wasm/       # wasm-bindgen exports for pulse
│   └── anvaya-simulator/        # noisy simulator CLI & HTTP server
├── shared/
│   └── types/                   # (generated) TypeScript types
├── scripts/
│   └── build-wasm.sh
├── agents.md                    # instructions for LLM agents working on this repo
└── nx.json                      # monorepo task orchestration
```

## Building Everything

```bash
# Build all WASM packages & Studio
pnpm nx run-many --target=build --all
```

## Running Tests

```bash
# All Rust tests
cargo test --manifest-path rust/Cargo.toml --all

# All JS/TS tests (including Playwright E2E)
cd apps/studio && pnpm test
```

## Contributing

We use a feature‑branch workflow:

1. Create a feature branch from `develop`.
2. Commit your changes (follow Conventional Commits).
3. Open a pull request to `develop`.
4. After review & CI passes, merge into `develop`.
5. Only `main` receives tagged releases.

See `agents.md` for internal tooling instructions.

## License

This repository is dual‑licensed:

- Rust crates & WebAssembly packages (all code under `rust/` and the generated npm packages in `packages/`) are licensed under the GNU Affero General Public License v3.0.
- Studio frontend (`apps/studio/`) is licensed under the Apache License 2.0.

See `LICENSE` for the full legal text.
