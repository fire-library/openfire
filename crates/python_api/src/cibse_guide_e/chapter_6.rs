pub mod appendix;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import CIBSE Guide E chapter 6 functions
use ::openfire::cibse_guide_e::chapter_6::{
    equation_6_7 as rust_equation_6_7, equation_6_55 as rust_equation_6_55,
    equation_6_57 as rust_equation_6_57, equation_6_58 as rust_equation_6_58,
};

// Equation 6_7 module functions
#[pyfunction]
/// Calculates the heat release rate at flashover (Equation 6.7).
///
/// This equation determines the minimum heat release rate required to achieve flashover
/// conditions in a compartment fire, based on the ventilation characteristics.
///
/// .. math::
///
///    Q_f = 600 \cdot A_{vo} \sqrt{H_o}
///
/// where:
///
/// - :math:`Q_f` is the heat release rate at flashover (kW)
/// - :math:`A_{vo}` is the ventilation factor area (m²)
/// - :math:`H_o` is the height of opening (m)
///
/// Args:
///     a_vo (float): Ventilation factor area (m²)
///     h_o (float): Height of opening (m)
///
/// Returns:
///     float: Heat release rate at flashover (kW)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_6.equation_6_7.heat_release_rate_flashover(2.0, 2.1)
fn heat_release_rate_flashover(a_vo: f64, h_o: f64) -> PyResult<f64> {
    Ok(rust_equation_6_7::heat_release_rate_flashover(a_vo, h_o))
}

#[pymodule]
/// Equation 6.7 - Heat Release Rate at Flashover.
///
/// Calculates the minimum heat release rate required to achieve flashover
/// conditions in a compartment fire based on ventilation characteristics.
fn equation_6_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(heat_release_rate_flashover, m)?)?;
    Ok(())
}

// Equation 6_55 module functions
#[pyfunction]
/// Calculates the mean flame height (Equation 6.55).
///
/// This equation determines the average height of flames in a compartment fire
/// based on the total heat release rate.
///
/// .. math::
///
///    Z_f = 0.2 \cdot Q_t^{2/5}
///
/// where:
///
/// - :math:`Z_f` is the mean flame height (m)
/// - :math:`Q_t` is the total heat release rate (kW)
///
/// Args:
///     q_t (float): Total heat release rate (kW)
///
/// Returns:
///     float: Mean flame height (m)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_6.equation_6_55.mean_flame_height(1000.0)
fn mean_flame_height(q_t: f64) -> PyResult<f64> {
    Ok(rust_equation_6_55::mean_flame_height(q_t))
}

#[pymodule]
/// Equation 6.55 - Mean Flame Height.
///
/// Calculates the average height of flames in a compartment fire
/// based on the total heat release rate.
fn equation_6_55(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean_flame_height, m)?)?;
    Ok(())
}

// Equation 6_57 module functions
#[pyfunction]
/// Calculates the height of flame above opening (Equation 6.57).
///
/// This equation determines the height of flames extending above a window or door
/// opening during a compartment fire, based on the burning rate and opening geometry.
///
/// .. math::
///
///    Z_{fo} = 12.8 \cdot \left(\frac{R}{W}\right)^{2/3} - H_o
///
/// where:
///
/// - :math:`Z_{fo}` is the height of flame above opening (m)
/// - :math:`R` is the burning rate (kg/s)
/// - :math:`W` is the width of opening (m)
/// - :math:`H_o` is the height of opening (m)
///
/// Args:
///     r (float): Burning rate (kg/s)
///     w (float): Width of opening (m)
///     h_o (float): Height of opening (m)
///
/// Returns:
///     float: Height of flame above opening (m)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_6.equation_6_57.height_of_flame_aboveopening(0.2, 1.0, 2.1)
fn height_of_flame_aboveopening(r: f64, w: f64, h_o: f64) -> PyResult<f64> {
    Ok(rust_equation_6_57::height_of_flame_aboveopening(r, w, h_o))
}

#[pymodule]
/// Equation 6.57 - Height of Flame Above Opening.
///
/// Calculates the height of flames extending above a window or door
/// opening during a compartment fire.
fn equation_6_57(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(height_of_flame_aboveopening, m)?)?;
    Ok(())
}

// Equation 6_58 module functions
#[pyfunction]
/// Calculates the ventilation-controlled rate of burning (Equation 6.58).
///
/// This equation determines the burning rate when the fire is limited by the
/// available ventilation, considering the total area, opening characteristics,
/// and compartment geometry.
///
/// .. math::
///
///    R = 0.02 \cdot \left[(A_t - A_o) \cdot (A_o \sqrt{H_o}) \cdot \left(\frac{W}{D}\right)\right]^{1/2}
///
/// where:
///
/// - :math:`R` is the ventilation-controlled burning rate (kg/s)
/// - :math:`A_t` is the total internal surface area (m²)
/// - :math:`A_o` is the area of opening (m²)
/// - :math:`H_o` is the height of opening (m)
/// - :math:`W` is the width of opening (m)
/// - :math:`D` is the depth of compartment (m)
///
/// Args:
///     a_t (float): Total internal surface area (m²)
///     a_o (float): Area of opening (m²)
///     h_o (float): Height of opening (m)
///     w (float): Width of opening (m)
///     d (float): Depth of compartment (m)
///
/// Returns:
///     float: Ventilation-controlled burning rate (kg/s)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_6.equation_6_58.vent_controlled_rate_of_burning(45.0, 2.1, 2.1, 3.0, 4.0)
fn vent_controlled_rate_of_burning(a_t: f64, a_o: f64, h_o: f64, w: f64, d: f64) -> PyResult<f64> {
    Ok(rust_equation_6_58::vent_controlled_rate_of_burning(
        a_t, a_o, h_o, w, d,
    ))
}

#[pymodule]
/// Equation 6.58 - Ventilation-Controlled Rate of Burning.
///
/// Calculates the burning rate when the fire is limited by the
/// available ventilation in the compartment.
fn equation_6_58(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(vent_controlled_rate_of_burning, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 6 - Fire and Smoke.
///
/// This chapter contains equations for fire dynamics and smoke behavior
/// in compartment fires, including flashover conditions, flame heights,
/// and ventilation-controlled burning rates.
pub fn chapter_6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_6_7))?;
    m.add_wrapped(wrap_pymodule!(equation_6_55))?;
    m.add_wrapped(wrap_pymodule!(equation_6_57))?;
    m.add_wrapped(wrap_pymodule!(equation_6_58))?;
    m.add_wrapped(wrap_pymodule!(appendix::appendix))?;
    Ok(())
}
