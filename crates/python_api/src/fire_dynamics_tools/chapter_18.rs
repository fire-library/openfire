use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use ::openfire::fire_dynamics_tools::chapter_18::equation_18_1 as rust_equation_18_1;
use ::openfire::fire_dynamics_tools::chapter_18::equation_18_2 as rust_equation_18_2;
use ::openfire::fire_dynamics_tools::chapter_18::equation_18_3 as rust_equation_18_3;

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
/// - :math:`S` is the visibility (m)
/// - :math:`K` is proportionality constant (dimensionless)
/// - :math:`\alpha_m` is the specific extinction coefficient (m²/kg)
/// - :math:`m_p` is the mass concentration of particulates (kg/m³)
///
/// Args:
///     k (float): Proportionality constant (dimensionless)
///     alpha_m (float): Specific extinction coefficient (m²/kg)
///     m_p (float): Mass concentration of particulates (kg/m³)
///
/// Returns:
///     float: Visibility (m)
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

#[pyfunction]
/// Mass of particulates produced (Equation 18-3).
///
/// This equation calculates the total mass of particulates produced based on
/// the fuel mass burned and particulate yield.
///
/// .. math::
///
///    M_p = y_p \cdot M_f
///
/// where:
///
/// - :math:`M_p` is the total mass of particulates produced (lb)
/// - :math:`y_p` is the particulate yield (dimensionless)
/// - :math:`M_f` is the mass of fuel burned (lb)
///
/// Args:
///     M_f (float): Mass of fuel burned (lb)
///     y_p (float): Particulate yield (dimensionless)
///
/// Returns:
///     float: Total mass of particulates produced (kg)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_18.equation_18_3.mass_particulates_produced(2.0, 0.015)
fn mass_particulates_produced(m_f: f64, y_p: f64) -> PyResult<f64> {
    Ok(rust_equation_18_3::mass_particulates_produced(m_f, y_p))
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
/// Equation 18-3 - Mass of particulates produced.
///
/// This module contains calculations for the total mass of particulates
/// produced based on fuel mass and particulate yield.
fn equation_18_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mass_particulates_produced, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 18 - Visibility calculations.
///
/// This module contains visibility calculations through smoke from Chapter 18.
pub fn chapter_18(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_18_1))?;
    m.add_wrapped(wrap_pymodule!(equation_18_2))?;
    m.add_wrapped(wrap_pymodule!(equation_18_3))?;
    Ok(())
}
