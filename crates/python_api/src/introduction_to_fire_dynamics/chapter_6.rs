use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import introduction_to_fire_dynamics chapter 6 functions
use ::openfire::introduction_to_fire_dynamics::chapter_6::equation_6_32 as rust_equation_6_32;

// Equation 6_32 module functions
#[pyfunction]
/// Calculate time to ignition for thermally thick materials (Equation 6.32).
///
/// This equation calculates the time required for ignition of a thermally thick material
/// under constant radiative heat flux.
///
/// .. math::
///
///    t_{ig} = \frac{\pi}{4} \cdot k \cdot \rho \cdot c \cdot \frac{(T_{ig} - T_0)^2}{q_r^2}
///
/// where:
///
/// - :math:`t_{ig}` is the time to ignition (s)
/// - :math:`k` is the thermal conductivity (W/m·K)
/// - :math:`\rho` is the density (kg/m³)
/// - :math:`c` is the specific heat capacity (J/kg·K)
/// - :math:`T_{ig}` is the ignition temperature (°C)
/// - :math:`T_0` is the initial temperature (°C)
/// - :math:`q_r` is the radiative heat flux (W/m²)
///
/// Args:
///     k (float): Thermal conductivity (W/m·K)
///     rho (float): Density (kg/m³)
///     c (float): Specific heat capacity (J/kg·K)
///     temp_ig (float): Ignition temperature (°C)
///     temp_o (float): Initial temperature (°C)
///     q_r (float): Radiative heat flux (W/m²)
///
/// Returns:
///     float: Time to ignition (s)
fn time_to_ignition_thermally_thick(
    k: f64,
    rho: f64,
    c: f64,
    temp_ig: f64,
    temp_o: f64,
    q_r: f64,
) -> PyResult<f64> {
    Ok(rust_equation_6_32::time_to_ignition_thermally_thick(
        k, rho, c, temp_ig, temp_o, q_r,
    ))
}

#[pymodule]
/// Equation 6.32 - Time to ignition for thermally thick materials.
///
/// Provides calculation for ignition time under constant radiative heat flux.
fn equation_6_32(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(time_to_ignition_thermally_thick, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 6 - Ignition processes.
///
/// This chapter provides equations for analyzing ignition of materials
/// under various thermal conditions.
pub fn chapter_6_intro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_6_32))?;
    Ok(())
}
