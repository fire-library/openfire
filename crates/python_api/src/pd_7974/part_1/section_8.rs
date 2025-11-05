use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import all pd_7974 functions
use ::openfire::pd_7974::part_1::section_8::{
    equation_4 as rust_equation_4, equation_28 as rust_equation_28,
    equation_29 as rust_equation_29, equation_33 as rust_equation_33,
    equation_41 as rust_equation_41, equation_42 as rust_equation_42,
    equation_43 as rust_equation_43, equation_44 as rust_equation_44,
};

// Equation 28 module functions
#[pyfunction]
/// Calculate heat release rate for fuel-controlled fire (Equation 28).
///
/// This equation calculates the heat release rate for a fuel-controlled fire based on 
/// the total floor area and ventilation opening characteristics.
///
/// .. math::
///
///    Q_{fo} = 7.8 \cdot A_t + 378 \cdot A_v \cdot \sqrt{H_v}
///
/// where:
///
/// - :math:`Q_{fo}` is the heat release rate (kW)
/// - :math:`A_t` is the total floor area (m²)
/// - :math:`A_v` is the area of ventilation openings (m²)
/// - :math:`H_v` is the height of ventilation openings (m)
///
/// Args:
///     a_t (float): Total floor area (m²)
///     a_v (float): Area of ventilation openings (m²)
///     h_v (float): Height of ventilation openings (m)
///
/// Returns:
///     float: Heat release rate (kW)
fn q_fo(a_t: f64, a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_28::q_fo(a_t, a_v, h_v))
}

#[pymodule]
/// Equation 28 - Heat release rate for fuel-controlled fire.
///
/// Provides calculation for heat release rate based on floor area and ventilation openings.
fn equation_28(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_fo, m)?)?;
    Ok(())
}

// Equation 29 module functions
#[pyfunction]
/// Calculate heat release rate for fuel-controlled fire with heat of combustion (Equation 29).
///
/// This equation calculates the heat release rate for a fuel-controlled fire incorporating
/// the heat of combustion of the fuel material.
///
/// .. math::
///
///    Q_{fo} = 610 \cdot \sqrt{H_k \cdot A_t \cdot A_v \cdot \sqrt{H_v}}
///
/// where:
///
/// - :math:`Q_{fo}` is the heat release rate (kW)
/// - :math:`H_k` is the heat of combustion (MJ/kg)
/// - :math:`A_t` is the total floor area (m²)
/// - :math:`A_v` is the area of ventilation openings (m²)
/// - :math:`H_v` is the height of ventilation openings (m)
///
/// Args:
///     h_k (float): Heat of combustion (MJ/kg)
///     a_t (float): Total floor area (m²)
///     a_v (float): Area of ventilation openings (m²)
///     h_v (float): Height of ventilation openings (m)
///
/// Returns:
///     float: Heat release rate (kW)
#[pyo3(name = "q_fo")]
fn q_fo_29(h_k: f64, a_t: f64, a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_29::q_fo(h_k, a_t, a_v, h_v))
}

#[pymodule]
/// Equation 29 - Heat release rate with heat of combustion.
///
/// Provides calculation for heat release rate incorporating fuel heat of combustion.
fn equation_29(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_fo_29, m)?)?;
    Ok(())
}

// Equation 33 module functions
#[pyfunction]
/// Calculate maximum heat release rate for ventilation-controlled fire (Equation 33).
///
/// This equation calculates the maximum heat release rate when a fire is limited
/// by the available ventilation.
///
/// .. math::
///
///    Q_{max,vc} = 1500 \cdot A_v \cdot \sqrt{H_v}
///
/// where:
///
/// - :math:`Q_{max,vc}` is the maximum heat release rate for ventilation-controlled fire (kW)
/// - :math:`A_v` is the area of ventilation openings (m²)
/// - :math:`H_v` is the height of ventilation openings (m)
///
/// Args:
///     a_v (float): Area of ventilation openings (m²)
///     h_v (float): Height of ventilation openings (m)
///
/// Returns:
///     float: Maximum heat release rate (kW)
fn q_max_vc(a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_33::q_max_vc(a_v, h_v))
}

#[pymodule]
/// Equation 33 - Ventilation-controlled heat release rate.
///
/// Provides calculation for maximum heat release rate limited by ventilation.
fn equation_33(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_max_vc, m)?)?;
    Ok(())
}

// Equation 4 module functions
#[pyfunction]
/// Calculate maximum heat release rate for fuel-controlled fire (Equation 4).
///
/// This equation calculates the maximum heat release rate when a fire is controlled
/// by the available fuel rather than ventilation.
///
/// .. math::
///
///    Q_{max,fc} = A_f \cdot HRRPUA
///
/// where:
///
/// - :math:`Q_{max,fc}` is the maximum heat release rate for fuel-controlled fire (kW)
/// - :math:`A_f` is the floor area of fire (m²)
/// - :math:`HRRPUA` is the heat release rate per unit area (kW/m²)
///
/// Args:
///     a_f (float): Floor area of fire (m²)
///     hrrpua (float): Heat release rate per unit area (kW/m²)
///
/// Returns:
///     float: Maximum heat release rate (kW)
fn q_max_fc(a_f: f64, hrrpua: f64) -> PyResult<f64> {
    Ok(rust_equation_4::q_max_fc(a_f, hrrpua))
}

#[pymodule]
/// Equation 4 - Fuel-controlled heat release rate.
///
/// Provides calculation for maximum heat release rate limited by fuel availability.
fn equation_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_max_fc, m)?)?;
    Ok(())
}

// Equation 41 module functions
#[pyfunction]
/// Calculate maximum gas temperature (Equation 41).
///
/// This equation calculates the maximum gas temperature in a compartment fire
/// based on the opening factor.
///
/// .. math::
///
///    T_{g,max} = 6000 \cdot \frac{1 - e^{-0.1 \omega}}{\sqrt{\omega}}
///
/// where:
///
/// - :math:`T_{g,max}` is the maximum gas temperature (°C)
/// - :math:`\omega` is the opening factor (m^0.5)
///
/// Args:
///     omega (float): Opening factor (m^0.5)
///
/// Returns:
///     float: Maximum gas temperature (°C)
fn t_g_max(omega: f64) -> PyResult<f64> {
    Ok(rust_equation_41::t_g_max(omega))
}

#[pymodule]
/// Equation 41 - Maximum gas temperature.
///
/// Provides calculation for maximum gas temperature in compartment fires.
fn equation_41(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(t_g_max, m)?)?;
    Ok(())
}

// Equation 42 module functions
#[pyfunction]
/// Calculate opening factor (Equation 42).
///
/// This equation calculates the opening factor which relates the compartment
/// geometry to the ventilation characteristics.
///
/// .. math::
///
///    \omega = \frac{A_t}{A_v \cdot \sqrt{H_v}}
///
/// where:
///
/// - :math:`\omega` is the opening factor (m^0.5)
/// - :math:`A_t` is the total floor area (m²)
/// - :math:`A_v` is the area of ventilation openings (m²)
/// - :math:`H_v` is the height of ventilation openings (m)
///
/// Args:
///     a_t (float): Total floor area (m²)
///     a_v (float): Area of ventilation openings (m²)
///     h_v (float): Height of ventilation openings (m)
///
/// Returns:
///     float: Opening factor (m^0.5)
fn omega(a_t: f64, a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_42::omega(a_t, a_v, h_v))
}

#[pymodule]
/// Equation 42 - Opening factor.
///
/// Provides calculation for opening factor relating compartment geometry to ventilation.
fn equation_42(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(omega, m)?)?;
    Ok(())
}

// Equation 43 module functions
#[pyfunction]
/// Calculate gas temperature (Equation 43).
///
/// This equation calculates the actual gas temperature in a compartment fire
/// based on the maximum gas temperature and fuel load density parameter.
///
/// .. math::
///
///    T_g = T_{g,max} \cdot (1 - e^{-0.05 \psi})
///
/// where:
///
/// - :math:`T_g` is the gas temperature (°C)
/// - :math:`T_{g,max}` is the maximum gas temperature (°C)
/// - :math:`\psi` is the fuel load density parameter (dimensionless)
///
/// Args:
///     t_g_max (float): Maximum gas temperature (°C)
///     psi (float): Fuel load density parameter (dimensionless)
///
/// Returns:
///     float: Gas temperature (°C)
fn t_g(t_g_max: f64, psi: f64) -> PyResult<f64> {
    Ok(rust_equation_43::t_g(t_g_max, psi))
}

#[pymodule]
/// Equation 43 - Gas temperature.
///
/// Provides calculation for actual gas temperature based on maximum temperature and fuel load.
fn equation_43(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(t_g, m)?)?;
    Ok(())
}

// Equation 44 module functions
#[pyfunction]
/// Calculate fuel load density parameter (Equation 44).
///
/// This equation calculates the fuel load density parameter which relates
/// the fuel load to the compartment ventilation characteristics.
///
/// .. math::
///
///    \psi = \frac{m_e}{\sqrt{A_v \cdot A_t}}
///
/// where:
///
/// - :math:`\psi` is the fuel load density parameter (dimensionless)
/// - :math:`m_e` is the fuel load density (kg/m²)
/// - :math:`A_v` is the area of ventilation openings (m²)
/// - :math:`A_t` is the total floor area (m²)
///
/// Args:
///     m_e (float): Fuel load density (kg/m²)
///     a_v (float): Area of ventilation openings (m²)
///     a_t (float): Total floor area (m²)
///
/// Returns:
///     float: Fuel load density parameter (dimensionless)
fn psi(m_e: f64, a_v: f64, a_t: f64) -> PyResult<f64> {
    Ok(rust_equation_44::psi(m_e, a_v, a_t))
}

#[pymodule]
/// Equation 44 - Fuel load density parameter.
///
/// Provides calculation for fuel load density parameter relating fuel to ventilation characteristics.
fn equation_44(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(psi, m)?)?;
    Ok(())
}

#[pymodule]
/// Section 8 - Fire growth and heat release rate calculations.
///
/// This section provides various equations for calculating heat release rates,
/// temperatures, and fire growth parameters according to PD 7974 Part 1.
pub fn section_8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_28))?;
    m.add_wrapped(wrap_pymodule!(equation_29))?;
    m.add_wrapped(wrap_pymodule!(equation_33))?;
    m.add_wrapped(wrap_pymodule!(equation_4))?;
    m.add_wrapped(wrap_pymodule!(equation_41))?;
    m.add_wrapped(wrap_pymodule!(equation_42))?;
    m.add_wrapped(wrap_pymodule!(equation_43))?;
    m.add_wrapped(wrap_pymodule!(equation_44))?;
    Ok(())
}
