pub mod chapter_15;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// BS 9999 - Code of Practice for Fire Safety in Buildings.
///
/// This module provides calculations for fire safety in buildings
/// as specified in BS 9999, the UK code of practice for fire safety design,
/// management and use of buildings.
///
/// BS 9999 provides methodologies for fire safety engineering calculations
/// including egress calculations, fire safety systems design, and building
/// fire safety management.
///
/// Available modules:
///     chapter_15: Chapter 15 - Means of escape calculations
///
/// Example:
///     >>> import ofire
///     >>> ofire.bs9999.chapter_15
pub fn bs9999(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_15::chapter_15))?;
    Ok(())
}
