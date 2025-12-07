use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use ::openfire::fire_dynamics_tools::chapter_18::equation_18_1 as rust_equation_18_1;
use ::openfire::fire_dynamics_tools::chapter_18::equation_18_2 as rust_equation_18_2;

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

#[pyfunction]
/// Mass concentration of particulates (Equation 18-2).
///
/// This equation calculates the mass concentration of particulates based on
/// the total mass of particulates and the volume.
///
/// .. math::
///
///    m_p = \frac{M_p}{V}
///
/// where:
///
/// - :math:`m_p` is the mass concentration of particulates (lb/ft³)
/// - :math:`M_p` is the total mass of particulates produced (lb)
/// - :math:`V` is the volume (ft³)
///
/// Args:
///     m_p (float): Total mass of particulates produced (lb)
///     v (float): Volume (ft³)
///
/// Returns:
///     float: Mass concentration of particulates (lb/ft³)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_18.equation_18_2.concentration_particulates(0.059, 90000.0)
fn concentration_particulates(m_p: f64, v: f64) -> PyResult<f64> {
    Ok(rust_equation_18_2::concentration_particulates(m_p, v))
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
/// Equation 18-2 - Mass concentration of particulates.
///
/// This module contains calculations for mass concentration of particulates
/// based on total mass and volume.
fn equation_18_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(concentration_particulates, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 18 - Visibility calculations.
///
/// This module contains visibility calculations through smoke from Chapter 18.
pub fn chapter_18(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_18_1))?;
    m.add_wrapped(wrap_pymodule!(equation_18_2))?;
    Ok(())
}