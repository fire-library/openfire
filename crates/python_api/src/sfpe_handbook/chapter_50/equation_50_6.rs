use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import sfpe_handbook chapter 50 equation_50_6 functions
use openfire::sfpe_handbook::chapter_50::equation_50_6 as rust_equation_50_6;

#[pyfunction]
/// This function calculates upper limit pressure difference due to piston effect from the shaft to the building
///
/// .. math::
///
///    \Delta P_{usi} = \frac{\rho}{2} \cdot \left(\frac{A_s \cdot A_e \cdot u}{A_a \cdot A_{ir} \cdot C_c}\right)^2
///
/// where:
///
/// - :math:`\Delta P_{usi}` is the upper limit pressure difference (Pa)
/// - :math:`\rho` is the air density in hoistway (kg/m³)
/// - :math:`A_s` is the cross-sectional area of the shaft (m²)
/// - :math:`A_e` is the effective area (m²)
/// - :math:`U` is the elevator car velocity (m/s)
/// - :math:`A_a` is the free area around the elevator car (m²)
/// - :math:`A_{ir}` is the leakage area between building and lobby (m²)
/// - :math:`C_c` is the flow coefficient for flow around car (dimensionless)
///
/// Args:
///     rho (float): Air density in hoistway (kg/m³)
///     a_s (float): Cross-sectional area of the shaft (m²)
///     a_e (float): Effective area (m²)
///     U (float): Elevator car velocity (m/s)
///     a_a (float): Free area around the elevator car (m²)
///     a_ir (float): Leakage area between building and lobby (m²)
///     c_c (float): Flow coefficient for flow around car (dimensionless)
///
/// Returns:
///     float: Pressure difference (Pa)
///
/// Assumptions:
///     None stated.
///
/// Limitations:
///     The flow coefficient was determined experimentally at 0.94 for multiple cars and 0.83 for a single car.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_6.pressure_difference(0.8, 6.0, 5.0, 2.5, 1.0, 1.0, 0.84)
///     >>> print(f"{result:.2f} Pa")
fn pressure_difference(
    rho: f64,
    a_s: f64,
    a_e: f64,
    u: f64,
    a_a: f64,
    a_ir: f64,
    c_c: f64,
) -> PyResult<f64> {
    Ok(rust_equation_50_6::pressure_difference(
        rho, a_s, a_e, u, a_a, a_ir, c_c,
    ))
}

#[pymodule]
pub fn equation_50_6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pressure_difference, m)?)?;
    Ok(())
}
