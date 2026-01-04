use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use openfire::sfpe_handbook::chapter_50::equation_50_16 as rust_equation_50_16;

#[pyfunction]
/// Flow area factor for pressurization systems, relevant for the calculation of the maximum height limit (equation 50.15).
///
/// .. math::
///
///    F_r = 1 + \frac{A_{sb}^2 \times (T_b + 273)}{A_{bo}^2 \times (T_s + 273)}
///
/// where:
///
/// - :math:`F_r` is the flow area factor (dimensionless)
/// - :math:`A_{sb}` is the stairwell bottom opening area (m²)
/// - :math:`T_b` is the building interior temperature (°C)
/// - :math:`A_{bo}` is the building outside opening area (m²)
/// - :math:`T_s` is the stairwell temperature (°C)
///
/// Args:
///     a_sb (float): Stairwell bottom opening area (m²)
///     a_bo (float): Building outside opening area (m²)
///     t_b (float): Building interior temperature (°C)
///     t_s (float): Stairwell temperature (°C)
///
/// Returns:
///     float: Flow area factor (dimensionless)
///
/// Assumptions:
///     None stated.
///
/// Limitations:
///     Refer to the SFPE Handbook for details. To determine `A_{sb}` and `A_{bo}`, the specifc layout of the lobbies and stairwells as well as the leakage paths must be understood.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_16.factor(0.005, 0.005, 15.0, 5.0)
///     >>> print(f"{result:.6f}")
fn factor(a_sb: f64, a_bo: f64, t_b: f64, t_s: f64) -> PyResult<f64> {
    Ok(rust_equation_50_16::factor(a_sb, a_bo, t_b, t_s))
}

#[pymodule]
pub fn equation_50_16(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(factor, m)?)?;
    Ok(())
}
