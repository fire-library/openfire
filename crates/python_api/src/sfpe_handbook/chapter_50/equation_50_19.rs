use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use openfire::sfpe_handbook::chapter_50::equation_50_19 as rust_equation_50_19;

#[pyfunction]
/// Visibility in smoke at a point where mass concentration of fuel burned is known.
///
/// .. math::
///
///    S_i = \frac{K}{2.303 \times \delta m \times C_i}
///
/// where:
///
/// - :math:`S_i` is the visibility through smoke (m)
/// - :math:`K` is the Bouguer-Beer law constant (dimensionless)
/// - :math:`\delta m` is the mass extinction coefficient (m²/g)
/// - :math:`C_i` is the mass concentration of particulates (g/m³)
///
/// Args:
///     k (float): Bouguer-Beer law constant (dimensionless)
///     delta_m (float): Mass extinction coefficient (m²/g)
///     c_i (float): Mass concentration of particulates (g/m³)
///
/// Returns:
///     float: Visibility through smoke (m)
///
/// Assumptions:
///     The calculated visibility can be thought of as visibility if smoke is uniform.
///
/// Limitations:
///     See assumptions above. Assumes uniform smoke and, utilises the proportionality constant K, commonly taken as 8 for illumnated signs and 3 for non-illuminated.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_19.visibility(8.0, 0.22, 1.0)
///     >>> print(f"{result:.2f}")
fn visibility(k: f64, delta_m: f64, c_i: f64) -> PyResult<f64> {
    Ok(rust_equation_50_19::visibility(k, delta_m, c_i))
}

#[pymodule]
pub fn equation_50_19(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(visibility, m)?)?;
    Ok(())
}
