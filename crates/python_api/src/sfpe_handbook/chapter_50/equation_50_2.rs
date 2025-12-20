use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import sfpe_handbook chapter 50 equation_50_2 functions
use openfire::sfpe_handbook::chapter_50::equation_50_2 as rust_equation_50_2;

#[pyfunction]
/// Pressure difference between a fire compartment and its surroundings.
///
/// This function calculates the pressure difference between a fire compartment and its surroundings.
///
/// .. math::
///
///    \Delta P_{so} = 3460 \cdot \left(\frac{1}{T_0 + 273} - \frac{1}{T_f + 273}\right) \cdot z
///
/// where:
///
/// - :math:`\Delta P_{so}` is the pressure difference due to stack effect (Pa)
/// - :math:`T_0` is the temperature of the surroundings (°C)
/// - :math:`T_f` is the fire compartment temperature (°C)
/// - :math:`z` is the height above the neutral plane (m)
///
/// Args:
///     t_0 (float): Temperature of the surroundings (°C)
///     t_f (float): Temperature of the fire compartment (°C)
///     z (float): Height above neutral plane (m)
///
/// Returns:
///     float: Pressure difference due to stack effect (Pa)
///
/// Assumptions:
///     To be completed.
///
/// Limitations:
///    None stated.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_2.pressure_difference_stack_effect_alt(-18.0, 21.0, 30.0)
///     >>> print(f"{result:.2f} Pa")
fn pressure_difference_stack_effect_alt(t_0: f64, t_s: f64, z: f64) -> PyResult<f64> {
    Ok(rust_equation_50_2::pressure_difference_stack_effect_alt(t_0, t_s, z))
}

#[pyfunction]
/// LaTeX equation string for pressure difference due to stack effect (alternate).
///
/// Returns the LaTeX representation of Equation 50.2 with custom variable names.
///
/// Args:
///     delta_p_so (str): Symbol for pressure difference
///     t_0 (str): Symbol for outdoor temperature
///     t_s (str): Symbol for shaft temperature
///     z (str): Symbol for height
///
/// Returns:
///     str: LaTeX equation string
///
/// Example:
///     >>> import ofire
///     >>> eq = ofire.sfpe_handbook.chapter_50.equation_50_2.pressure_difference_stack_effect_alt_equation("ΔP", "T_out", "T_shaft", "h")
///     >>> print(eq)
fn pressure_difference_stack_effect_alt_equation(
    delta_p_so: String,
    t_0: String,
    t_s: String,
    z: String,
) -> PyResult<String> {
    Ok(rust_equation_50_2::pressure_difference_stack_effect_alt_equation(delta_p_so, t_0, t_s, z))
}

#[pymodule]
/// Equation 50.2 - Pressure difference due to stack effect (alternate formulation).
///
/// This module contains the alternate implementation of stack effect pressure difference
/// calculation from the SFPE Handbook, using the same formulation as Equation 50.1.
pub fn equation_50_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pressure_difference_stack_effect_alt, m)?)?;
    m.add_function(wrap_pyfunction!(pressure_difference_stack_effect_alt_equation, m)?)?;
    Ok(())
}