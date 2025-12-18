pub mod annex_e;
pub mod section_3;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
///
/// Example:
///     >>> import ofire
///     >>> ofire.eurocode_1_1_2.section_3
///     >>> ofire.eurocode_1_1_2.annex_e
pub fn eurocode_1_1_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(annex_e::annex_e))?;
    m.add_wrapped(wrap_pymodule!(section_3::section_3))?;
    Ok(())
}
