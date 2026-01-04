use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use openfire::sfpe_handbook::chapter_50::equation_50_15 as rust_equation_50_15;

#[pyfunction]
/// Height limit above which acceptable pressurization is not possible for an idealized building.
///
/// .. math::
///
///    H_m = 0.000289 \times \frac{F_R \times (\Delta p_{max} - \Delta p_{min})}{\left|\frac{1}{T_0 + 273} - \frac{1}{T_s + 273}\right|}
///
/// where:
///
/// - :math:`H_m` is the height limit (m)
/// - :math:`F_R` is the flow area factor (dimensionless)
/// - :math:`\Delta p_{max}` is the maximum design pressure difference (Pa)
/// - :math:`\Delta p_{min}` is the minimum design pressure difference (Pa)
/// - :math:`T_0` is the ambient temperature (°C)
/// - :math:`T_s` is the stairwell temperature (°C)
///
/// Args:
///     f_r (float): Flow area factor (dimensionless)
///     delta_p_max (float): Maximum design pressure difference (Pa)
///     delta_p_min (float): Minimum design pressure difference (Pa)
///     t_0 (float): Ambient temperature (°C)
///     t_s (float): Stairwell temperature (°C)
///
/// Returns:
///     float: Height limit (m)
///
/// Assumptions:
///     Standard atmospheric pressure at sea level is assumed.
///
/// Limitations:
///     This equation has been derived for idealised buildings. More likely to apply with systems with treated supply air.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_15.height_limit(2.0, 75.0, 25.0, 0.0, 25.0)
///     >>> print(f"{result:.6f}")
fn height_limit(f_r: f64, delta_p_max: f64, delta_p_min: f64, t_0: f64, t_s: f64) -> PyResult<f64> {
    Ok(rust_equation_50_15::height_limit(
        f_r,
        delta_p_max,
        delta_p_min,
        t_0,
        t_s,
    ))
}

#[pymodule]
pub fn equation_50_15(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(height_limit, m)?)?;
    Ok(())
}
