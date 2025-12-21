use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import sfpe_handbook chapter 50 equation_50_4 functions
use openfire::sfpe_handbook::chapter_50::equation_50_4 as rust_equation_50_4;

#[pyfunction]
/// This function calculates the pressure exerted by wind on a building's wall surface.
///
/// .. math::
///
///    P_w = 0.5 \cdot C_w \cdot \rho_0 \cdot u_h^2
///
/// where:
///
/// - :math:`P_w` is the pressure exerted by wind (Pa)
/// - :math:`C_w` is the wind pressure coefficient (dimensionless)
/// - :math:`\rho_0` is the air density at ambient conditions (kg/m³)
/// - :math:`u_h` is the wind speed at height h (m/s)
///
/// Args:
///     c_w (float): Wind pressure coefficient (dimensionless)
///     rho_0 (float): Air density at ambient conditions (kg/m³)
///     u_h (float): Wind speed at height h (m/s)
///
/// Returns:
///     float: Pressure exerted by wind (Pa)
///
/// Assumptions:
///     The pressure coefficient `C_w` depends on wind direction, building geometry and local obstructions.
///
/// Limitations:
///     None stated.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_4.pressure_exerted_wind(0.8, 1.2, 15.0)
///     >>> print(f"{result:.2f} Pa")
fn wind_pressure(c_w: f64, rho_0: f64, u_h: f64) -> PyResult<f64> {
    Ok(rust_equation_50_4::wind_pressure(c_w, rho_0, u_h))
}

#[pymodule]
pub fn equation_50_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wind_pressure, m)?)?;
    Ok(())
}
