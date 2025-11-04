pub mod part_1;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// PD 7974 - Application of fire safety engineering principles to the design of buildings.
///
/// This module provides calculations and tools based on PD 7974, the UK standard
/// for applying fire safety engineering principles to building design.
///
/// Available submodules:
///     part_1: Part 1 calculations
///
/// Example:
///     >>> import ofire
///     >>> ofire.pd_7974.part_1
pub fn pd_7974(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(part_1::part_1))?;
    Ok(())
}
