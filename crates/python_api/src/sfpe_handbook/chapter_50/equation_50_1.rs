use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use openfire::sfpe_handbook::chapter_50::equation_50_1 as rust_equation_50_1;

#[pyfunction]
/// Pressure difference due to stack effect.
///
/// This function calculates the pressure difference between indoor and outdoor
/// environments due to stack effect.
///
/// .. math::
///
///    \Delta P_{so} = 3460 \cdot \left(\frac{1}{T_0 + 273} - \frac{1}{T_s + 273}\right) \cdot z
///
/// where:
///
/// - :math:`\Delta P_{so}` is the pressure difference due to stack effect (Pa)
/// - :math:`T_0` is the outdoor temperature (°C)
/// - :math:`T_s` is the shaft temperature (°C)
/// - :math:`z` is the height above the neutral pressure level (m)
///
/// Args:
///     t_0 (float): Outdoor temperature (°C)
///     t_s (float): Shaft temperature (°C)
///     z (float): Height above neutral pressure level (m)
///
/// Returns:
///     float: Pressure difference due to stack effect (Pa)
///
/// Assumptions:
///     To be completed.
///
/// Limitations:
///     Not applicable for complicated buildings, with shafts of various heights
///     and different shafts temperatures.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_1.pressure_difference(-18.0, 21.0, 30.0)
///     >>> print(f"{result:.2f} Pa")
fn pressure_difference(t_0: f64, t_s: f64, z: f64) -> PyResult<f64> {
    Ok(rust_equation_50_1::pressure_difference(t_0, t_s, z))
}

#[pymodule]
pub fn equation_50_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pressure_difference, m)?)?;
    Ok(())
}
