use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import all eurocode_1_1_2 section 3 functions
use ::openfire::eurocode_1_1_2::section_3::{
    equation_3_1 as rust_equation_3_1,
    equation_3_2 as rust_equation_3_2,
    equation_3_3 as rust_equation_3_3,
    equation_3_4 as rust_equation_3_4,
    equation_3_5 as rust_equation_3_5,
    equation_3_6 as rust_equation_3_6,
};

// Equation 3.1 module functions
#[pyfunction]
/// Net heat flux per unit area of the surface.
///
/// This equation determines the total net heat flux to a surface by combining
/// convective and radiative components.
///
/// Args:
///     h_net_c (float): Net convective heat flux per unit area (W/m²)
///     h_net_r (float): Net radiative heat flux per unit area (W/m²)
///
/// Returns:
///     float: Net heat flux per unit area (W/m²)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.eurocode_1_1_2.section_3.equation_3_1.net_heat_flux_surface(15000.0, 25000.0)
fn net_heat_flux_surface(h_net_c: f64, h_net_r: f64) -> f64 {
    rust_equation_3_1::net_heat_flux_surface(h_net_c, h_net_r)
}

#[pymodule]
/// Equation 3.1 - Net heat flux per unit area of the surface.
pub fn equation_3_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(net_heat_flux_surface, m)?)?;
    Ok(())
}

// Equation 3.2 module functions
#[pyfunction]
/// Net convective heat flux per unit area of the surface.
///
/// This equation calculates the net convective heat flux to a surface
/// based on the heat transfer coefficient and temperature difference.
///
/// Args:
///     alpha_c (float): Heat transfer coefficient (W/m²K)
///     delta_g (float): Gas temperature in the vicinity of the exposed member (°C)
///     delta_m (float): Member surface temperature (°C)
///
/// Returns:
///     float: Net convective heat flux per unit area (W/m²)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.eurocode_1_1_2.section_3.equation_3_2.net_convective_heat_flux_surface(50.0, 650.0, 150.0)
fn net_convective_heat_flux_surface(alpha_c: f64, delta_g: f64, delta_m: f64) -> f64 {
    rust_equation_3_2::net_convective_heat_flux_surface(alpha_c, delta_g, delta_m)
}

#[pymodule]
/// Equation 3.2 - Net convective heat flux per unit area of the surface.
pub fn equation_3_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(net_convective_heat_flux_surface, m)?)?;
    Ok(())
}

// Equation 3.3 module functions
#[pyfunction]
/// Net radiative heat flux per unit area of the surface.
///
/// This equation calculates the net radiative heat flux to a surface
/// considering configuration factor, material properties, and temperature difference.
///
/// Args:
///     phi (float): Configuration factor (dimensionless)
///     epsilon_m (float): Surface emissivity of the member (dimensionless)
///     epsilon_f (float): Emissivity of the fire (dimensionless)
///     sigma (float): Stefan-Boltzmann constant (W/m²K⁴)
///     delta_r (float): Effective radiation temperature of the fire environment (°C)
///     delta_m (float): Surface temperature of the member (°C)
///
/// Returns:
///     float: Net radiative heat flux per unit area (W/m²)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.eurocode_1_1_2.section_3.equation_3_3.net_radiative_heat_flux_surface(0.8, 0.8, 0.9, 5.67e-8, 650.0, 150.0)
fn net_radiative_heat_flux_surface(phi: f64, epsilon_m: f64, epsilon_f: f64, sigma: f64, delta_r: f64, delta_m: f64) -> f64 {
    rust_equation_3_3::net_radiative_heat_flux_surface(phi, epsilon_m, epsilon_f, sigma, delta_r, delta_m)
}

#[pymodule]
/// Equation 3.3 - Net radiative heat flux per unit area of the surface.
pub fn equation_3_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(net_radiative_heat_flux_surface, m)?)?;
    Ok(())
}

// Equation 3.4 module functions
#[pyfunction]
/// Standard temperature-time curve calculation.
///
/// Args:
///     t (float): Time (minutes)
///
/// Returns:
///     float: Temperature (°C)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.eurocode_1_1_2.section_3.equation_3_4.standard_temp_time_curve(10.0)
fn standard_temp_time_curve(t: f64) -> f64 {
    rust_equation_3_4::standard_temp_time_curve(t)
}

#[pymodule]
/// Equation 3.4 - Standard temperature-time curve calculation.
pub fn equation_3_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(standard_temp_time_curve, m)?)?;
    Ok(())
}

// Equation 3.5 module functions
#[pyfunction]
/// External temperature-time curve calculation.
///
/// Args:
///     t (float): Time (minutes)
///
/// Returns:
///     float: Temperature (°C)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.eurocode_1_1_2.section_3.equation_3_5.external_temp_time_curve(10.0)
fn external_temp_time_curve(t: f64) -> f64 {
    rust_equation_3_5::external_temp_time_curve(t)
}

#[pymodule]
/// Equation 3.5 - External temperature-time curve calculation.
pub fn equation_3_5(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(external_temp_time_curve, m)?)?;
    Ok(())
}

// Equation 3.6 module functions
#[pyfunction]
/// Hydrocarbon temperature-time curve calculation.
///
/// Args:
///     t (float): Time (minutes)
///
/// Returns:
///     float: Temperature (°C)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.eurocode_1_1_2.section_3.equation_3_6.hydrocarbon_temp_time_curve(10.0)
fn hydrocarbon_temp_time_curve(t: f64) -> f64 {
    rust_equation_3_6::hydrocarbon_temp_time_curve(t)
}

#[pymodule]
/// Equation 3.6 - Hydrocarbon temperature-time curve calculation.
pub fn equation_3_6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hydrocarbon_temp_time_curve, m)?)?;
    Ok(())
}


#[pymodule]
/// Section 3 - Placeholder section for demonstration.
///
/// This section contains placeholder equations for demonstration purposes.
pub fn section_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_3_1))?;
    m.add_wrapped(wrap_pymodule!(equation_3_2))?;
    m.add_wrapped(wrap_pymodule!(equation_3_3))?;
    m.add_wrapped(wrap_pymodule!(equation_3_4))?;
    m.add_wrapped(wrap_pymodule!(equation_3_5))?;
    m.add_wrapped(wrap_pymodule!(equation_3_6))?;

    Ok(())
}