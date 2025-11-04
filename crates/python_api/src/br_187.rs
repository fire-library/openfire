pub mod appendix_a;
pub mod chapter_1;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// BR 187 - External Fire Spread to Buildings.
///
/// This module provides calculations for external fire spread to buildings
/// as specified in BR 187, the UK guidance document for external fire spread.
///
/// BR 187 provides methodologies for assessing the risk of fire spread between
/// buildings and calculating thermal radiation exposure.
///
/// Available modules:
///     chapter_1: Chapter 1 calculations
///     appendix_a: Appendix A thermal radiation calculations
///
/// Example:
///     >>> import ofire
///     >>> ofire.br_187.chapter_1
///     >>> ofire.br_187.appendix_a
pub fn br_187(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(appendix_a::appendix_a))?;
    m.add_wrapped(wrap_pymodule!(chapter_1::chapter_1))?;
    Ok(())
}
