---
applyTo: "crates/python_api/**"
---

## Copilot – Python API docstring and binding guide (python_api only)

Scope: This file applies when editing code under `crates/python_api/` only. Follow these steps; search the codebase only if something here is missing or incorrect.

### What this crate is

- Python bindings for the Rust `openfire` library using PyO3/maturin.
- Publishes a Python module named `ofire` exposing fire engineering calculations.
- Docstrings written in Rust `///` comments become Python docstrings and are rendered in Sphinx docs.

### Build and validate (always do this after edits)

1. Create/activate venv and install tools (first time per machine):
   - `python -m venv .venv && source .venv/bin/activate`
   - `pip install --upgrade pip maturin`
2. From repo root (or this crate): build/editable install:
   - `cd crates/python_api && maturin develop`
3. Quick import smoke test:
   - `python -c "import ofire; print(ofire.__name__)"` (expect `ofire`)
4. Optional docs build (requires `pip install -r docs/docs-requirements.txt` in the venv):
   - `cd ..../repo-root && sphinx-build -b html docs _build`

Notes:

- The crate uses `crate-type = ["cdylib"]` and depends on root `openfire` via path; workspace build is handled by maturin.
- If builds are slow or fail due to stale artifacts, run `cargo clean` at repo root and rerun `maturin develop`.

### Standard docstring format (use consistently)

Place the docstring above the PyO3 function in Rust with `///`. Use reStructuredText with a math section and clear Args/Returns.

Example:

```rust
#[pyfunction]
/// Radiation intensity (Equation A.1).
///
/// Computes surface radiation intensity from emissivity and temperature.
///
/// .. math::
///
///    I_s = \sigma\,\varepsilon\,T^4
///
/// where:
///
/// - :math:`\sigma` is the Stefan–Boltzmann constant (kW·m⁻²·K⁻⁴)
/// - :math:`\varepsilon` is the surface emissivity (dimensionless)
/// - :math:`T` is the absolute temperature (K)
///
/// Args:
///     emissivity (float): Surface emissivity (0–1, dimensionless)
///     temperature (float): Absolute temperature (K)
///
/// Returns:
///     float: Radiation intensity (kW/m²)
///
/// Example:
///     >>> import ofire
///     >>> ofire.br_187.radiation_intensity(0.9, 293.15)
pub fn radiation_intensity(emissivity: f64, temperature: f64) -> PyResult<f64> {
    // ... implementation ...
}
```

Formatting rules:

- Always include a `.. math::` block for equations, followed by a `where:` list of symbol definitions.
- Leave a blank line after `.. math::` and after `where:` so Sphinx renders correctly.
- Use `:math:` inline role for symbols in lists; include units, or state “dimensionless”.
- Args/Returns use Python-facing types: `float`, `int`, `bool`, `str`.

### Verification workflow (to ensure accuracy)

1. Locate the source equation in the corresponding Rust crate under `crates/{document}/src/` and confirm the formula/units.
2. Keep LaTeX symbols and variable names aligned with the engineering documents and Rust implementation.
3. Run `maturin develop` and import `ofire`; spot-check 1–2 values against tests or known results.
4. If building docs, ensure `maturin develop` ran first so `ofire` can be imported by Sphinx.

### PyO3 conventions to follow

- Functions: `#[pyfunction]` returning `PyResult<T>`; map Rust types to Python-friendly ones (e.g., `f64 -> float`).
- Modules: `#[pymodule]` with `pub fn module_name(m: &Bound<'_, PyModule>) -> PyResult<()>` and `m.add_function(wrap_pyfunction!(.., m)?)?`.
- Optional text signatures: add `#[pyo3(text_signature = "(emissivity, temperature)")]` for nicer `help()` output.
- Keep modules and function names snake_case for Python ergonomics.

### File locations you will touch

- Bindings and docstrings: `crates/python_api/src/`
- Rust equations (reference only): `crates/<document>/src/`
- Docs (optional build): `docs/` → output `_build/`

### Common pitfalls (and fixes)

- Missing blank lines around `.. math::` or `where:` → malformed rendering. Add blank lines exactly as in the example.
- Units omitted or inconsistent → add units to each symbol and to Returns.
- Docstring formula diverges from code → update LaTeX to match the implemented equation.
- Import error `ModuleNotFoundError: ofire` during docs build → run `maturin develop` in the active venv first.

### Quick checklist before committing

- `maturin develop` succeeds and `python -c "import ofire"` works in the venv.
- Docstrings include `.. math::`, a `where:` section, Args, Returns, and an Example.
- Symbols/units match the Rust implementation and source document.

Trust these instructions and only search if something behaves differently on your machine or a step is missing.
