pub mod section_2;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// TR 17 - Technical Report calculations and guidance.
///
/// This module contains equations and calculation methods from TR 17
/// technical report, focusing on fire dynamics principles and
/// non-dimensional analysis for fire engineering applications.
pub fn tr_17(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(section_2::section_2))?;
    Ok(())
}
