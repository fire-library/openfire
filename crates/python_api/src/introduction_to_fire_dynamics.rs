pub mod chapter_10;
pub mod chapter_6;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// Introduction to Fire Dynamics - Fundamental fire behavior calculations.
///
/// This module provides fundamental equations and calculations for understanding
/// fire behavior, ignition processes, and compartment fire dynamics.
pub fn introduction_to_fire_dynamics(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_6::chapter_6_intro))?;
    m.add_wrapped(wrap_pymodule!(chapter_10::chapter_10_intro))?;
    Ok(())
}
