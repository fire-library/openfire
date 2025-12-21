use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import sfpe_handbook chapter 50 equation_50_7 functions
use openfire::sfpe_handbook::chapter_50::equation_50_7 as rust_equation_50_7;

#[pyfunction]
/// This function calculates the effective area for smoke control systems (Equation 50-7).
///
/// .. math::
///
///    A_{effective} = \left(\frac{1}{A_{sr}^2} + \frac{1}{A_{ir}^2} + \frac{1}{A_{io}^2}\right)^{-0.5}
///
/// where:
///
/// - :math:`A_{effective}` is the effective area (m²)
/// - :math:`A_{sr}` is the smoke removal area (m²)
/// - :math:`A_{ir}` is the air intake/return area (m²)
/// - :math:`A_{io}` is the inlet/outlet area (m²)
///
/// Args:
///     a_sr (float): Smoke removal area (m²)
///     a_ir (float): Air intake/return area (m²)
///     a_io (float): Inlet/outlet area (m²)
///
/// Returns:
///     float: Effective area (m²)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_7.effective_area(0.5, 0.75, 2.5)
///     >>> print(f"{result:.6f} m²")
fn effective_area(a_sr: f64, a_ir: f64, a_io: f64) -> PyResult<f64> {
    Ok(rust_equation_50_7::effective_area(a_sr, a_ir, a_io))
}

#[pymodule]
pub fn equation_50_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(effective_area, m)?)?;
    Ok(())
}
