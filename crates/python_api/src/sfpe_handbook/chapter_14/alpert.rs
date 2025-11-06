pub mod heat_release;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// Alpert Correlations - Ceiling jet correlations for fire properties.
///
/// This module contains correlations developed by Alpert for ceiling jet
/// flows beneath unconfined ceilings. These correlations relate fire
/// properties to measurements taken in the ceiling jet.
pub fn alpert(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(heat_release::heat_release))?;
    Ok(())
}
