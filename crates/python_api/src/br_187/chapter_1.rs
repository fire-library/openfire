use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import BR_187 chapter 1 functions
use ::openfire::br_187::chapter_1::equation_1 as rust_equation_1;

// Equation 1 module functions
#[pyfunction]
/// Calculate ventilation factor for external fire spread assessment (Equation 1).
/// 
/// This function calculates the ventilation factor used in BR 187 methodologies
/// for assessing external fire spread between buildings.
/// 
/// Args:
///     a_s: Area of external wall surface (m²)
///     a: Area of openings in the external wall (m²)
///     h: Height of openings (m)
/// 
/// Returns:
///     Ventilation factor (dimensionless)
/// 
/// Example:
///     >>> import ofire
///     >>> factor = ofire.br_187.chapter_1.equation_1.calculate_ventilation_factor(100.0, 20.0, 2.5)
fn calculate_ventilation_factor(a_s: f64, a: f64, h: f64) -> PyResult<f64> {
    Ok(rust_equation_1::calculate_ventilation_factor(a_s, a, h))
}

#[pymodule]
/// BR 187 Chapter 1 Equation 1 - Ventilation factor calculations.
/// 
/// This module provides functions for calculating ventilation factors
/// used in external fire spread assessments.
fn equation_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_ventilation_factor, m)?)?;
    Ok(())
}

#[pymodule]
/// BR 187 Chapter 1 - External fire spread fundamentals.
/// 
/// This chapter provides fundamental calculations for external fire spread
/// assessment including ventilation factors and building geometry considerations.
/// 
/// Available equations:
///     equation_1: Ventilation factor calculations
pub fn chapter_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_1))?;
    Ok(())
}
