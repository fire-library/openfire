use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use openfire::sfpe_handbook::chapter_50::equation_50_17 as rust_equation_50_17;

#[pyfunction]
/// Stairwell temperature calculation for untreated pressurization air.
///
/// .. math::
///
///    T_S = T_0 + \eta (T_B - T_0)
///
/// where:
///
/// - :math:`T_S` is the stairwell temperature (°C)
/// - :math:`T_0` is the outdoors temperature (°C)  
/// - :math:`\eta` is the heat transfer factor (dimensionless)
/// - :math:`T_B` is the building temperature (°C)
///
/// Args:
///     t_0 (float): Outdoors temperature (°C)
///     eta (float): Heat transfer factor (dimensionless)
///     t_b (float): Building temperature (°C)
///
/// Returns:
///     float: Stairwell temperature (°C)
///
/// Assumptions:
///     None stated.
///
/// Limitations:
///     This equation applies to untreated pressurization air. A heat transfer factor of :math:`\eta = 0.15` is suggested in the absence of better data.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_17.stairwell_temperature(-10.0, 0.15, 15.0)
///     >>> print(f"{result:.2f}")
fn stairwell_temperature(t_0: f64, eta: f64, t_b: f64) -> PyResult<f64> {
    Ok(rust_equation_50_17::stairwell_temperature(t_0, eta, t_b))
}

#[pymodule]
pub fn equation_50_17(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stairwell_temperature, m)?)?;
    Ok(())
}