use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import introduction_to_fire_dynamics chapter 10 functions
use openfire::introduction_to_fire_dynamics::chapter_10::{equation_10_18 as rust_equation_10_18};

// Equation 10_18 module functions
#[pyfunction]
/// Calculate the ventilation parameter for compartment fires (Equation 10.18).
///
/// This equation calculates a dimensionless parameter used to determine
/// the burning regime of a compartment fire.
///
/// .. math::
///
///    N = \frac{\rho \sqrt{g} A_w \sqrt{H}}{A_f}
///
/// where:
///
/// - :math:`N` is the ventilation parameter (dimensionless)
/// - :math:`\rho` is the density of air (kg/m³)
/// - :math:`g` is the acceleration due to gravity (m/s²)
/// - :math:`A_w` is the area of window/vent opening (m²)
/// - :math:`H` is the height of window/vent opening (m)
/// - :math:`A_f` is the floor area (m²)
///
/// Args:
///     rho (float): Density of air (kg/m³)
///     g (float): Acceleration due to gravity (m/s²)
///     a_w (float): Area of window/vent opening (m²)
///     h (float): Height of window/vent opening (m)
///     a_f (float): Floor area (m²)
///
/// Returns:
///     float: Ventilation parameter (dimensionless)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
fn calculate(rho: f64, g: f64, a_w: f64, h: f64, a_f: f64) -> PyResult<f64> {
    Ok(rust_equation_10_18::calculate(rho, g, a_w, h, a_f))
}

#[pyfunction]
/// Determine the burning regime based on ventilation parameter (Equation 10.18).
///
/// This function determines whether a compartment fire is ventilation-controlled,
/// fuel-controlled, or in a transition regime based on the calculated ventilation parameter.
///
/// **Regime Classification:**
///
/// - **Ventilation Controlled**: N < 0.235
///   
///   Fire is limited by available air/oxygen supply
///
/// - **Transition/Crossover**: 0.235 ≤ N ≤ 0.290
///   
///   Fire is in a transitional state between ventilation and fuel control
///
/// - **Fuel Controlled**: N > 0.290
///   
///   Fire is limited by available fuel, adequate ventilation exists
///
/// Args:
///     number (float): Ventilation parameter (dimensionless)
///
/// Returns:
///     str: Burning regime classification
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
fn heating_regime(number: f64) -> PyResult<String> {
    let regime = rust_equation_10_18::heating_regime(number);
    Ok(regime.to_string())
}

#[pymodule]
/// Equation 10.18 - Ventilation parameter and burning regime determination.
///
/// Provides calculations to determine compartment fire burning regimes.
fn equation_10_18(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;
    m.add_function(wrap_pyfunction!(heating_regime, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 10 - Compartment fire dynamics.
///
/// This chapter provides equations for analyzing compartment fire behavior
/// and determining burning regimes.
pub fn chapter_10_intro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_10_18))?;
    Ok(())
}
