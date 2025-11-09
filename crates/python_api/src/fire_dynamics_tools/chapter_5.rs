use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use ::openfire::fire_dynamics_tools::chapter_5::equation_5_1 as rust_equation_5_1;

#[pyfunction]
/// Calculate the result of equation 5-1 (Equation 5.1).
///
/// This equation calculates a fraction-based result using four input parameters.
/// The equation represents a proportional relationship between the product of
/// the first two parameters and the product of the last two parameters.
///
/// .. math::
///
///    R = \frac{a \cdot b}{c \cdot d}
///
/// where:
///
/// - :math:`R` is the calculated result (dimensionless)
/// - :math:`a` is the first parameter (dimensionless)
/// - :math:`b` is the second parameter (dimensionless)
/// - :math:`c` is the third parameter (dimensionless)
/// - :math:`d` is the fourth parameter (dimensionless)
///
/// Args:
///     a (float): First parameter (dimensionless)
///     b (float): Second parameter (dimensionless)
///     c (float): Third parameter (dimensionless)
///     d (float): Fourth parameter (dimensionless)
///
/// Returns:
///     float: The calculated result (dimensionless)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_5.equation_5_1.calculate_equation_5_1(10.0, 5.0, 2.0, 3.0)
fn calculate_equation_5_1(a: f64, b: f64, c: f64, d: f64) -> PyResult<f64> {
    Ok(rust_equation_5_1::calculate_equation_5_1(a, b, c, d))
}

#[pymodule]
/// Equation 5-1 calculations.
///
/// This module contains calculations for equation 5-1, which computes
/// a fraction-based result from four input parameters.
fn equation_5_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_equation_5_1, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 5 - Fire dynamics calculations.
///
/// This module contains fire dynamics calculations from Chapter 5.
pub fn chapter_5(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_5_1))?;
    Ok(())
}