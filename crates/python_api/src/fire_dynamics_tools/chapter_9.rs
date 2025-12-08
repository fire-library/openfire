use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use ::openfire::fire_dynamics_tools::chapter_9::equation_9_2 as rust_equation_9_2;
use ::openfire::fire_dynamics_tools::chapter_9::equation_9_3 as rust_equation_9_3;
use ::openfire::fire_dynamics_tools::chapter_9::equation_9_4 as rust_equation_9_4;

#[pyfunction]
/// Maximum centerline temperature rise in a plume above a fire source (Equation 9-2).
///
/// This equation calculates the maximum temperature rise along the centerline
/// of a fire plume at a given height above the fire source.
///
/// .. math::
///
///    \Delta T_p = 9.1 \left(\frac{T_a}{g \cdot c_p^2 \cdot \rho_a^2}\right)^{1/3} \frac{\dot{Q}_c^{2/3}}{(z - z_0)^{5/3}}
///
/// where:
///
/// - :math:`\Delta T_p` is the maximum centerline temperature rise (K)
/// - :math:`T_a` is the ambient temperature (K)
/// - :math:`\dot{Q}_c` is the convective heat release rate (kW)
/// - :math:`g` is the acceleration of gravity (m/s²)
/// - :math:`c_p` is the specific heat of air (kJ/kg·K)
/// - :math:`\rho_a` is the density of ambient air (kg/m³)
/// - :math:`z` is elevation above the fire source (m)
/// - :math:`z_0` is the hypothetical virtual origin of the fire (m)
///
/// Args:
///     t_a (float): Ambient temperature (K)
///     q_c (float): Convective heat release rate (kW)
///     g (float): Acceleration of gravity (m/s²)
///     c_p (float): Specific heat of air (kJ/kg·K)
///     rho_a (float): Density of ambient air (kg/m³)
///     z (float): Elevation above fire source (m)
///     z_o (float): Hypothetical virtual origin of the fire (m)
///
/// Returns:
///     float: Maximum centerline temperature rise (K)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_9.equation_9_2.maximum_centerline_temperature_plume(288.0, 700.0, 9.8, 1.0, 1.2, 2.0, -0.25)
fn maximum_centerline_temperature_rise_plume(
    t_a: f64,
    q_c: f64,
    g: f64,
    c_p: f64,
    rho_a: f64,
    z: f64,
    z_o: f64,
) -> PyResult<f64> {
    Ok(
        rust_equation_9_2::maximum_centerline_temperature_rise_plume(
            t_a, q_c, g, c_p, rho_a, z, z_o,
        ),
    )
}

#[pyfunction]
/// Virtual origin height normalized by fire diameter (Equation 9-3).
///
/// This equation calculates the ratio of virtual origin height to fire diameter
/// for use in fire plume calculations.
///
/// .. math::
///
///    \frac{z_0}{D} = -1.02 + 0.083 \frac{\dot{Q}^{2/5}}{D}
///
/// where:
///
/// - :math:`z_0` is the virtual origin height (m)
/// - :math:`D` is the fire diameter (m)
/// - :math:`\dot{Q}` is the total heat release rate (kW)
///
/// Args:
///     d (float): Fire diameter (m)
///     q (float): Total heat release rate (kW)
///
/// Returns:
///     float: Virtual origin height to diameter ratio (dimensionless)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_9.equation_9_3.virtual_origin_over_diameter(2.2, 750.0)
fn virtual_origin_over_diameter(d: f64, q: f64) -> PyResult<f64> {
    Ok(rust_equation_9_3::virtual_origin_over_diameter(d, q))
}

#[pyfunction]
/// Effective diameter of a fire source from its area (Equation 9-4).
///
/// This equation calculates the effective diameter of a fire source based on
/// its area, assuming an equivalent circular fire source.
///
/// .. math::
///
///    D = \left(\frac{4 \cdot A_f}{\pi}\right)^{1/2}
///
/// where:
///
/// - :math:`D` is the effective diameter (m)
/// - :math:`A_f` is the fire area (m²)
///
/// Args:
///     a_f (float): Fire area (m²)
///
/// Returns:
///     float: Effective diameter (m)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_9.equation_9_4.effective_diameter(4.0)
fn effective_diameter(a_f: f64) -> PyResult<f64> {
    Ok(rust_equation_9_4::effective_diameter(a_f))
}

#[pymodule]
/// Equation 9-2 - Maximum centerline temperature rise in fire plumes.
///
/// This module contains calculations for the maximum temperature rise along
/// the centerline of a fire plume at a given height above the fire source.
fn equation_9_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(
        maximum_centerline_temperature_rise_plume,
        m
    )?)?;
    Ok(())
}

#[pymodule]
/// Equation 9-3 - Virtual origin height to diameter ratio.
///
/// This module contains calculations for the virtual origin height to diameter
/// ratio used in fire plume calculations.
fn equation_9_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(virtual_origin_over_diameter, m)?)?;
    Ok(())
}

#[pymodule]
/// Equation 9-4 - Effective diameter of a fire source.
///
/// This module contains calculations for the effective diameter of a fire source
/// based on its area, assuming an equivalent circular fire source.
fn equation_9_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(effective_diameter, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 9 - Fire plume temperature calculations.
///
/// This module contains fire plume temperature calculations from Chapter 9.
pub fn chapter_9(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_9_2))?;
    m.add_wrapped(wrap_pymodule!(equation_9_3))?;
    m.add_wrapped(wrap_pymodule!(equation_9_4))?;
    Ok(())
}
