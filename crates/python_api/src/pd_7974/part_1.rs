pub mod section_8;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// PD 7974 Part 1 - Initiation and development of fire within the enclosure of origin.
///
/// This module contains calculations for fire initiation and development
/// within the enclosure of origin, as specified in PD 7974 Part 1.
///
/// Available sections:
///     section_8: Section 8 calculations
pub fn part_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(section_8::section_8))?;
    Ok(())
}
