use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use openfire::sfpe_handbook::chapter_50::equation_50_20 as rust_equation_50_20;

#[pyfunction]
/// Visibility calculation through smoke from percent obscuration.
///
/// .. math::
///
///    S_i = -\frac{K \times L}{\ln(1 - \frac{\lambda}{100})}
///
/// where:
///
/// - :math:`S_i` is the visibility through smoke (m)
/// - :math:`K` is proportionality constant (dimensionless)
/// - :math:`L` is the path length (m)
/// - :math:`\lambda` is the percent obscuration (dimensionless)
///
/// Args:
///     k (float): Proportionality constant (dimensionless)
///     l (float): Path length (m)
///     lambda (float): Percent obscuration (dimensionless)
///
/// Returns:
///     float: Visibility through smoke (m)
///
/// Assumptions:
///     An object can be seen for S > L.
///
/// Limitations:
///     None stated.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_20.visibility(8.0, 10.0, 95.0)
///     >>> print(f"{result:.2f}")
fn visibility(k: f64, l: f64, lambda: f64) -> PyResult<f64> {
    Ok(rust_equation_50_20::visibility(k, l, lambda))
}

#[pymodule]
pub fn equation_50_20(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(visibility, m)?)?;
    Ok(())
}