pub mod alpert;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// Chapter 14 - Fire Detection correlations and calculations.
///
/// This chapter contains correlations and methods for fire detection
/// systems, including ceiling jet correlations and related fire
/// property calculations.
pub fn chapter_14(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(alpert::alpert))?;
    Ok(())
}
