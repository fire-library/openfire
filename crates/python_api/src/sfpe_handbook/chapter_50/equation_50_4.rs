use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import sfpe_handbook chapter 50 equation_50_4 functions
use openfire::sfpe_handbook::chapter_50::equation_50_4 as rust_equation_50_4;

#[pyfunction]
/// Pressure difference between a fire compartment and its surroundings - alternate formulation (Equation 50.4).
///
/// This function calculates the pressure difference between a fire compartment and its surroundings,
/// using the same formulation as Equation 50.2.
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
///     - Static pressure conditions apply
///     - Temperatures are uniform in each zone
///     - Air behaves as an ideal gas
///     - No wind effects or mechanical ventilation interference
///     - Neutral pressure level exists and is known
///
/// Limitations:
///     - Valid for typical building temperature ranges (-50°C to 1000°C)
///     - Does not account for dynamic pressure effects
///     - Assumes negligible air leakage effects on temperature distribution
///     - Limited to vertical pressure differences only
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_4.pressure_difference_fire_compartment_alt(20.0, 800.0, 1.52)
///     >>> print(f"{result:.2f} Pa")
fn pressure_difference_fire_compartment_alt(t_0: f64, t_f: f64, z: f64) -> PyResult<f64> {
    Ok(rust_equation_50_4::pressure_difference_fire_compartment_alt(t_0, t_f, z))
}

#[pyfunction]
/// LaTeX equation string for pressure difference between fire compartment and surroundings (alternate).
///
/// Returns the LaTeX representation of Equation 50.4 with custom variable names.
///
/// Args:
///     delta_p_so (str): Symbol for pressure difference
///     t_0 (str): Symbol for temperature of surroundings
///     t_f (str): Symbol for fire compartment temperature
///     z (str): Symbol for height
///
/// Returns:
///     str: LaTeX equation string
///
/// Example:
///     >>> import ofire
///     >>> eq = ofire.sfpe_handbook.chapter_50.equation_50_4.pressure_difference_fire_compartment_alt_equation("ΔP", "T_amb", "T_fire", "h")
///     >>> print(eq)
fn pressure_difference_fire_compartment_alt_equation(
    delta_p_so: String,
    t_0: String,
    t_f: String,
    z: String,
) -> PyResult<String> {
    Ok(rust_equation_50_4::pressure_difference_fire_compartment_alt_equation(delta_p_so, t_0, t_f, z))
}

#[pymodule]
/// Equation 50.4 - Pressure difference between a fire compartment and its surroundings (alternate).
///
/// This module contains the alternate implementation of pressure difference calculation
/// between a fire compartment and its surroundings, using the same formulation as Equation 50.2.
pub fn equation_50_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pressure_difference_fire_compartment_alt, m)?)?;
    m.add_function(wrap_pyfunction!(pressure_difference_fire_compartment_alt_equation, m)?)?;
    Ok(())
}