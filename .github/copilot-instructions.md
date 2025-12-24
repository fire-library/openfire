## OpenFire – coding agent onboarding

This document gives a first-time coding agent everything needed to work efficiently in this repo. Trust these steps; only search if something here is missing or proves incorrect.

## What this repo is

- Purpose: Fire-safety engineering calculations implemented in Rust, organized by source documents (BR 187, BS 9999, CIBSE Guide E, PD 7974, SFPE Handbook, TR 17, Introduction to Fire Dynamics) with optional Python bindings and Sphinx docs.
- Type/size: Cargo workspace with multiple library crates; top-level Rust library re-exports domain crates. Includes a Python extension crate and Sphinx docs. Target runtime: Rust stable; Python 3.8+ via PyO3/maturin.
- Primary languages: Rust, Python (bindings/docs). CI runs on Ubuntu; local development validated on macOS.

## Tech stack snapshot (validated locally)

- Rust toolchain: rustc 1.85.0, cargo 1.85.0 (stable). Workspace builds and unit tests pass.
- Python: 3.8+ recommended (abi3). Local test used 3.14.0 with maturin 1.6+.
- Docs: Sphinx >= 7 with furo theme and autodoc-typehints.

## Bootstrap once per machine

Always set up a Python virtual environment before docs or Python bindings work.

```bash
# from repo root
/usr/bin/python3 -m venv .venv
source .venv/bin/activate
pip install --upgrade pip
pip install -r crates/python_api/docs/docs-requirements.txt
pip install maturin
```

Notes:

- No system packages are needed on macOS to build/test Rust. CI installs some Ubuntu libs (webkit, appindicator, rsvg, patchelf) but they are not required for unit tests here.

## Build, test, run – Rust

Preconditions: Rust stable toolchain installed.

Commands that work reliably from repo root:

- Build the workspace:
  - `cargo build`
- Compile tests without running (fast feedback):
  - `cargo test --workspace --no-run`
- Run unit tests only (recommended for local iterations):
  - `cargo test --workspace --lib`
- Full test run (matches CI behavior):
  - `cargo test --workspace --verbose`

Observed behavior and workarounds:

- On macOS, a doctest in `crates/br_187` can fail due to Unicode math in a doc code block. If you hit a doctest failure during local runs, re-run with `--lib` to iterate quickly. Before opening a PR, prefer keeping doctests non-compiling snippets fenced as text (```text) or annotated with `ignore` in the code comment to prevent compilation.

Useful maintenance:

- Format: `cargo fmt` (optional but recommended before commits).
- Lint: `cargo clippy --workspace --all-targets -- -D warnings` (not enforced by CI but helpful).
- Clean build: `cargo clean` (if caching issues suspected).
- Coverage: `cargo llvm-cov --html --output-dir crates/python_api/docs/_static/coverage --workspace --exclude python_api` to generate test coverage reports.

## Python bindings (PyO3/maturin)

The Python package lives in `crates/python_api` and builds a module named `ofire`.

- Editable install for local dev (ensures Rust code is built and importable in venv):

  - From repo root, with venv active:
    - `cd crates/python_api && maturin develop`
  - Quick validation:
    - `python -c "import ofire; print(ofire.__name__)"`

- Build wheels (like CI):
  - `maturin build --release -m crates/python_api/pyproject.toml`

Notes:

- The crate `crates/python_api/Cargo.toml` uses `crate-type = ["cdylib"]` and depends on the root `openfire` crate via a path dependency.

## Documentation (Sphinx)

Docs live under `crates/python_api/docs/`. API docs import the Python module built via `maturin develop`.

Steps to build locally:

```bash
source .venv/bin/activate
cd crates/python_api && maturin develop
sphinx-build -b html docs _build
```

Observed warnings (harmless):

- `html_static_path entry '_static' does not exist` – fine if you don’t use custom static assets.
- A reStructuredText title underline length warning in `docs/index.rst` and an `unknown document: '../api'` reference – docs still build.

## CI and release checks

GitHub Actions workflows:

- `.github/workflows/test.yaml` – cargo test on Ubuntu stable Rust; restores Cargo caches; installs some system libs; runs `cargo test --workspace --verbose`.
- `.github/workflows/docs.yaml` – sets up Python 3.11 and Rust; installs docs deps; runs `maturin develop` for `ofire`; builds Sphinx HTML and deploys to GitHub Pages on main/master.
- `.github/workflows/python-release.yaml` – builds wheels via `PyO3/maturin-action` for Linux, Windows, and macOS targets, plus sdist, then uploads to PyPI on `release` branch with OIDC.

To replicate CI locally:

- Tests: `cargo test --workspace --verbose` (see doctest note above).
- Docs: follow the “Documentation” steps with an active venv and `maturin develop`.
- Python wheels: run `maturin build` (a matrix of targets requires OS-specific toolchains; local builds will produce host wheels).

## Project layout – where to make changes

- Root:
  - `Cargo.toml` – workspace definition and root `openfire` crate dependencies on domain crates.
  - `src/lib.rs` – re-exports the domain crates: `pub use br_187;`, `bs9999;`, `cibse_guide_e;`, `fire_dynamics_tools;`, `introduction_to_fire_dynamics;`, `pd_7974;`, `sfpe_handbook;`, `tr17;`.
- Domain crates (under `crates/`): `br_187/`, `bs9999/`, `cibse_guide_e/`, `fire_dynamics_tools/`, `introduction_to_fire_dynamics/`, `pd_7974/`, `sfpe_handbook/`, `tr17/` — each is a Rust library crate with its own `Cargo.toml` and `src/` containing the domain equations and tests.
- Python binding: `crates/python_api/` – `pyproject.toml`, `Cargo.toml` (cdylib), and Rust module exposing Python APIs.
- Docs: `crates/python_api/docs/` – Sphinx config (`conf.py`), `index.rst`, `guide/`, `api/` . Built site goes to `crates/python_api/_build/`.
- PDFs: `Documents/` – reference documents used to implement formulas (not used by builds/tests).

Hidden gotchas and dependencies:

- Doctest Unicode: non-Rust snippets in doc comments should be fenced as `text` or marked `ignore` to avoid doctest compilation errors.
- macOS SSL warning from urllib3 during docs build may appear (LibreSSL vs OpenSSL); currently harmless.

## Quick file map (root level)

- `Cargo.toml`, `Cargo.lock` – workspace and dependency locks.
- `src/lib.rs` – re-export hub.
- `crates/` – all Rust crates including `python_api/`.
- `crates/python_api/docs/`, `crates/python_api/_build/` – Sphinx sources and generated site.
- `.github/workflows/` – `test.yaml`, `docs.yaml`, `python-release.yaml`.
- `README.md` – brief overview of workspace.

## Test Coverage Requirements

All calculation functions that perform fire safety engineering computations **MUST** have 100% test coverage. This includes mathematical equation implementations, fire dynamics calculations, heat transfer calculations, and smoke movement algorithms.

**Excluding code from coverage**: Use `#[cfg(not(coverage))]` to mark functions that should not be included in coverage analysis, such as equation functions that return string representations of mathematical equations.

Example:
```rust
// This function returns a string representation and can be excluded
#[cfg(not(coverage))]
pub fn equation_string() -> &'static str {
    "Q = m * c_p * ΔT"
}

// This calculation function MUST have 100% test coverage
pub fn heat_transfer(mass: f64, specific_heat: f64, temp_diff: f64) -> f64 {
    mass * specific_heat * temp_diff
}
```

## Success checklist before opening a PR

- `cargo build` and `cargo test --workspace` (or `--lib` if iterating due to doctest warning) pass locally.
- If touching Python binding or docs: venv active, `maturin develop` succeeds, `sphinx-build` succeeds.
- All calculation functions have 100% test coverage (check with `cargo llvm-cov --workspace --exclude python_api`).
- Optional: `cargo fmt` and `cargo clippy` clean locally.

Prefer following these instructions verbatim. Search the codebase only if something here is incomplete or behaves differently in your environment.
