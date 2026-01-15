use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use openfire::sfpe_handbook::chapter_50::equation_50_18 as rust_equation_50_18;

#[pyfunction]
/// Fractional Effective Dose (FED) calculation for evaluation of exposure to smoke.
///
/// .. math::
///
///    FED = \frac{\sum (C_i \Delta t_i)}{LC_{t50}}
///
/// where:
///
/// - :math:`FED` is the fractional effective dose (dimensionless)
/// - :math:`C_i` is the mass concentration of material burned at the end of time interval i (g/m³)
/// - :math:`\Delta t_i` is the time interval i (s)
/// - :math:`LC_{t50}` is the lethal exposure dose from test data (mg/m³ or ppm)
///
/// Args:
///     c_i (list[float]): Concentration values at each time interval (g/m³)
///     delta_t_i (float): Time intervals (s)
///     lc_t50 (float): Lethal exposure dose from test data (mg/m³ or ppm)
///
/// Returns:
///     float: Fractional effective dose (dimensionless)
///
/// Assumptions:
///     Uniform time intervals.
///
/// Limitations:
///     Simplest model for evaluating exposure to smoke.
///
/// Example:
///     >>> import ofire
///     >>> c_i = [0.001, 0.002, 0.003]
///     >>> delta_t_i = [1.0, 1.0, 1.0]
///     >>> lc_t50 = 10.0
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_18.fed(c_i, delta_t_i, lc_t50)
///     >>> print(f"{result:.2f}")
fn fed(c_i: Vec<f64>, delta_t_i: f64, lc_t50: f64) -> PyResult<f64> {
    Ok(rust_equation_50_18::fed(c_i, delta_t_i, lc_t50))
}

#[pymodule]
pub fn equation_50_18(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fed, m)?)?;
    Ok(())
}
