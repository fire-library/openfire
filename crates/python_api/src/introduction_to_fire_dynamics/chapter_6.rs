use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import introduction_to_fire_dynamics chapter 6 functions
use ::openfire::introduction_to_fire_dynamics::chapter_6::equation_6_32 as rust_equation_6_32;
use ::openfire::introduction_to_fire_dynamics::chapter_6::equation_6_33 as rust_equation_6_33;

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

// Equation 6_33 module functions
#[pyfunction]
/// Calculate time to ignition for thermally thin materials (Equation 6.33).
///
/// This equation calculates the time required for ignition of a thermally thin material
/// under constant radiative heat flux.
///
/// .. math::
///
///    t_{ig} = \rho \cdot c \cdot \tau \cdot (T_{ig} - T_0) \cdot \frac{1}{q_r}
///
/// where:
///
/// - :math:`t_{ig}` is the time to ignition (s)
/// - :math:`\rho` is the density (kg/m³)
/// - :math:`c` is the specific heat capacity (J/kg·K)
/// - :math:`\tau` is the thickness (m)
/// - :math:`T_{ig}` is the ignition temperature (K)
/// - :math:`T_0` is the initial temperature (K)
/// - :math:`q_r` is the radiative heat flux (W/m²)
///
/// Args:
///     rho (float): Density (kg/m³)
///     c (float): Specific heat capacity (J/kg·K)
///     tau (float): Thickness (m)
///     temp_ig (float): Ignition temperature (K)
///     temp_o (float): Initial temperature (K)
///     q_r (float): Radiative heat flux (W/m²)
///
/// Returns:
///     float: Time to ignition (s)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.introduction_to_fire_dynamics.chapter_6_intro.equation_6_33.time_to_ignition_thermally_thin(1190.0, 1420.0, 0.001, 573.0, 298.0, 20000.0)
///     >>> print(f"{result:.5f} s")
fn time_to_ignition_thermally_thin(
    rho: f64,
    c: f64,
    tau: f64,
    temp_ig: f64,
    temp_o: f64,
    q_r: f64,
) -> PyResult<f64> {
    Ok(rust_equation_6_33::time_to_ignition_thermally_thin(
        rho, c, tau, temp_ig, temp_o, q_r,
    ))
}

#[pymodule]
/// Equation 6.33 - Time to ignition for thermally thin materials.
///
/// Provides calculation for ignition time of thin materials under constant radiative heat flux.
fn equation_6_33(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(time_to_ignition_thermally_thin, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 6 - Ignition processes.
///
/// This chapter provides equations for analyzing ignition of materials
/// under various thermal conditions.
pub fn chapter_6_intro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_6_32))?;
    m.add_wrapped(wrap_pymodule!(equation_6_33))?;
    Ok(())
}
