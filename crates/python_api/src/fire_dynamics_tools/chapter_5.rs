use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use ::openfire::fire_dynamics_tools::chapter_5::equation_5_1 as rust_equation_5_1;

#[pyfunction]
/// This equation calculates the thermal radiation incident flux from a point
/// source at a given distance, accounting for the radiative fraction of the
/// total heat release rate.
///
/// .. math::
///
///    \dot{q}^{"} = \frac{\chi_r \cdot \dot{Q}}{4 \cdot \pi \cdot r^2}
///
/// where:
///
/// - :math:`\dot{q}''` is the radiant heat flux (kW/m²)
/// - :math:`\dot{Q}` is the heat release rate of the fire (kW)
/// - :math:`r` is the radial distance from the center of the flame to the edge of the target (m)
/// - :math:`\chi_r` is the fraction of total energy radiated (dimensionless)
///
/// Args:
///     q (float): Heat release rate (kW)
///     r (float): Radial distance (m)
///     x_r (float): Radiative fraction (dimensionless)
///
/// Returns:
///     float: Radiant heat flux (kW/m²)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_5.equation_5_1.thermal_radiation_point_source(750.0, 2.5, 0.3)
fn thermal_radiation_point_source(q: f64, r: f64, x_r: f64) -> PyResult<f64> {
    Ok(rust_equation_5_1::thermal_radiation_point_source(q, r, x_r))
}

#[pymodule]
/// This module contains calculations for thermal radiation incident flux
/// from a point source fire.
fn equation_5_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(thermal_radiation_point_source, m)?)?;
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
