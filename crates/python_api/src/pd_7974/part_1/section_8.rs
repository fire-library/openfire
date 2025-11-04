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
/// Args:
///     a_t: Total floor area (m²)
///     a_v: Area of ventilation openings (m²)  
///     h_v: Height of ventilation openings (m)
///
/// Returns:
///     Heat release rate (kW)
fn q_fo(a_t: f64, a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_28::q_fo(a_t, a_v, h_v))
}

#[pymodule]
/// PD 7974 Part 1 Section 8 Equation 28 - Heat release rate calculations.
fn equation_28(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_fo, m)?)?;
    Ok(())
}

// Equation 29 module functions
#[pyfunction]
/// Calculate heat release rate for fuel-controlled fire (Equation 29).
///
/// Args:
///     h_k: Heat of combustion (MJ/kg)
///     a_t: Total floor area (m²)
///     a_v: Area of ventilation openings (m²)
///     h_v: Height of ventilation openings (m)
///
/// Returns:
///     Heat release rate (kW)
#[pyo3(name = "q_fo")]
fn q_fo_29(h_k: f64, a_t: f64, a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_29::q_fo(h_k, a_t, a_v, h_v))
}

#[pymodule]
/// PD 7974 Part 1 Section 8 Equation 29 - Heat release rate calculations with heat of combustion.
fn equation_29(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_fo_29, m)?)?;
    Ok(())
}

// Equation 33 module functions
#[pyfunction]
/// Calculate maximum heat release rate for ventilation-controlled fire (Equation 33).
///
/// Args:
///     a_v: Area of ventilation openings (m²)
///     h_v: Height of ventilation openings (m)
///
/// Returns:
///     Maximum heat release rate (kW)
fn q_max_vc(a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_33::q_max_vc(a_v, h_v))
}

#[pymodule]
/// PD 7974 Part 1 Section 8 Equation 33 - Ventilation-controlled heat release rate calculations.
fn equation_33(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_max_vc, m)?)?;
    Ok(())
}

// Equation 4 module functions
#[pyfunction]
/// Calculate maximum heat release rate for fuel-controlled fire (Equation 4).
///
/// Args:
///     a_f: Floor area of fire (m²)
///     hrrpua: Heat release rate per unit area (kW/m²)
///
/// Returns:
///     Maximum heat release rate (kW)
fn q_max_fc(a_f: f64, hrrpua: f64) -> PyResult<f64> {
    Ok(rust_equation_4::q_max_fc(a_f, hrrpua))
}

#[pymodule]
/// PD 7974 Part 1 Section 8 Equation 4 - Fuel-controlled heat release rate calculations.
fn equation_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_max_fc, m)?)?;
    Ok(())
}

// Equation 41 module functions
#[pyfunction]
/// Calculate maximum gas temperature (Equation 41).
///
/// Args:
///     omega: Opening factor (m^0.5)
///
/// Returns:
///     Maximum gas temperature (°C)
fn t_g_max(omega: f64) -> PyResult<f64> {
    Ok(rust_equation_41::t_g_max(omega))
}

#[pymodule]
/// PD 7974 Part 1 Section 8 Equation 41 - Maximum gas temperature calculations.
fn equation_41(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(t_g_max, m)?)?;
    Ok(())
}

// Equation 42 module functions
#[pyfunction]
/// Calculate opening factor (Equation 42).
///
/// Args:
///     a_t: Total floor area (m²)
///     a_v: Area of ventilation openings (m²)
///     h_v: Height of ventilation openings (m)
///
/// Returns:
///     Opening factor (m^0.5)
fn omega(a_t: f64, a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_42::omega(a_t, a_v, h_v))
}

#[pymodule]
/// PD 7974 Part 1 Section 8 Equation 42 - Opening factor calculations.
fn equation_42(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(omega, m)?)?;
    Ok(())
}

// Equation 43 module functions
#[pyfunction]
/// Calculate gas temperature (Equation 43).
///
/// Args:
///     t_g_max: Maximum gas temperature (°C)
///     psi: Fuel load density parameter
///
/// Returns:
///     Gas temperature (°C)
fn t_g(t_g_max: f64, psi: f64) -> PyResult<f64> {
    Ok(rust_equation_43::t_g(t_g_max, psi))
}

#[pymodule]
/// PD 7974 Part 1 Section 8 Equation 43 - Gas temperature calculations.
fn equation_43(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(t_g, m)?)?;
    Ok(())
}

// Equation 44 module functions
#[pyfunction]
/// Calculate fuel load density parameter (Equation 44).
///
/// Args:
///     m_e: Fuel load density (kg/m²)
///     a_v: Area of ventilation openings (m²)
///     a_t: Total floor area (m²)
///
/// Returns:
///     Fuel load density parameter (dimensionless)
fn psi(m_e: f64, a_v: f64, a_t: f64) -> PyResult<f64> {
    Ok(rust_equation_44::psi(m_e, a_v, a_t))
}

#[pymodule]
/// PD 7974 Part 1 Section 8 Equation 44 - Fuel load density parameter calculations.
fn equation_44(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(psi, m)?)?;
    Ok(())
}

#[pymodule]
/// PD 7974 Part 1 Section 8 - Fire growth and heat release rate calculations.
///
/// This section provides various equations for calculating heat release rates,
/// temperatures, and fire growth parameters according to PD 7974 Part 1.
///
/// Available submodules:
///     equation_28: Heat release rate for fuel-controlled fire (Equation 28)
///     equation_29: Heat release rate with heat of combustion (Equation 29)
///     equation_33: Ventilation-controlled heat release rate (Equation 33)
///     equation_4: Fuel-controlled heat release rate (Equation 4)
///     equation_41: Maximum gas temperature (Equation 41)
///     equation_42: Opening factor (Equation 42)
///     equation_43: Gas temperature (Equation 43)
///     equation_44: Fuel load density parameter (Equation 44)
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
