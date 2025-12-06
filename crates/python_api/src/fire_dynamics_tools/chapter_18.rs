use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use ::openfire::fire_dynamics_tools::chapter_18::equation_18_1 as rust_equation_18_1;

#[pyfunction]
/// Visibility through smoke (Equation 18-1).
///
/// This equation calculates the visibility through smoke based on
/// the extinction coefficient and mass concentration of particulates.
///
/// .. math::
///
///    S = \frac{K}{\alpha_m \cdot m_p}
///
/// where:
///
/// - :math:`S` is the visibility (ft)
/// - :math:`K` is proportionality constant (dimensionless)
/// - :math:`\alpha_m` is the specific extinction coefficient (ft²/lb)
/// - :math:`m_p` is the mass concentration of particulates (lb/ft³)
///
/// Args:
///     k (float): Proportionality constant (dimensionless)
///     alpha_m (float): Specific extinction coefficient (ft²/lb)
///     m_p (float): Mass concentration of particulates (lb/ft³)
///
/// Returns:
///     float: Visibility (ft)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_18.equation_18_1.visibility(8.0, 37000.0, 0.000006)
fn visibility(k: f64, alpha_m: f64, m_p: f64) -> PyResult<f64> {
    Ok(rust_equation_18_1::visibility(k, alpha_m, m_p))
}

#[pymodule]
/// Equation 18-1 - Visibility through smoke.
///
/// This module contains calculations for visibility distance through smoke
/// based on extinction coefficients and particulate mass concentrations.
fn equation_18_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(visibility, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 18 - Visibility calculations.
///
/// This module contains visibility calculations through smoke from Chapter 18.
pub fn chapter_18(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_18_1))?;
    Ok(())
}