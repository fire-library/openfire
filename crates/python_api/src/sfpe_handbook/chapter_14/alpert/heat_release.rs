use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import sfpe_handbook chapter 14 alpert heat_release functions
use ::openfire::sfpe_handbook::chapter_14::alpert::heat_release as rust_heat_release;

#[pyfunction]
/// Calculate heat release rate from temperature and position using Alpert correlations.
///
/// This function implements the Alpert correlations for estimating heat release rate
/// from ceiling jet temperature measurements at different radial positions beneath
/// an unconfined ceiling. These correlations are fundamental for fire detection and
/// suppression system design.
///
/// .. math::
///    :nowrap:
///
///    \begin{align}
///    \text{For } \frac{r}{H} \leq 0.18: \quad &\dot{Q} = \left(\frac{(T - T_{\infty}) \cdot H^{5/3}}{16.9}\right)^{3/2} \\
///    \text{For } \frac{r}{H} > 0.18: \quad &\dot{Q} = \left(\frac{(T - T_{\infty}) \cdot \left(\frac{r}{H}\right)^{2/3} \cdot H^{5/3}}{5.38}\right)^{3/2}
///    \end{align}
///
/// where:
///
/// - :math:`\dot{Q}` is the heat release rate (kW)
/// - :math:`T` is the ceiling jet temperature (K)
/// - :math:`T_{\infty}` is the ambient temperature (K)
/// - :math:`H` is the height of the ceiling above the fire (m)
/// - :math:`r` is the radial distance from the fire centerline (m)
///
/// Args:
///     temp (float): Ceiling jet temperature (K)
///     temp_amb (float): Ambient temperature (K)
///     height (float): Height of ceiling above fire (m)
///     radial_position (float): Radial distance from fire centerline (m)
///
/// Returns:
///     float: Heat release rate (kW)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> # Calculate heat release rate for ceiling jet at 500K, 5m high, 2m from center
///     >>> q = ofire.sfpe_handbook.chapter_14.alpert.heat_release.from_temperature_and_position(
///     ...     temp=500.0,
///     ...     temp_amb=293.15,
///     ...     height=5.0,
///     ...     radial_position=2.0
///     ... )
///     >>> print(f"Heat release rate: {q:.1f} kW")
fn from_temperature_and_position(
    temp: f64,
    temp_amb: f64,
    height: f64,
    radial_position: f64,
) -> PyResult<f64> {
    Ok(rust_heat_release::from_temperature_and_position(
        temp,
        temp_amb,
        height,
        radial_position,
    ))
}

#[pymodule]
/// Heat Release Rate Correlations - Alpert correlations for estimating heat release rates.
///
/// This module contains the Alpert correlations for calculating heat release rates
/// from ceiling jet temperature measurements. The correlations distinguish between
/// two regions based on the normalized radial distance (r/H).
pub fn heat_release(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_temperature_and_position, m)?)?;
    Ok(())
}
