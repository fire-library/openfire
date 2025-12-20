pub mod equation_50_1;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// This chapter contains equations for calculating pressure differences
/// due to stack effect in buildings and smoke movement calculations.
pub fn chapter_50(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_50_1::equation_50_1))?;
    Ok(())
}