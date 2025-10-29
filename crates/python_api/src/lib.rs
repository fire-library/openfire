use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import all pd_7974 functions
use ::openfire::pd_7974::part_1::section_8::{
    equation_28 as rust_equation_28, equation_29 as rust_equation_29, 
    equation_33 as rust_equation_33, equation_4 as rust_equation_4, 
    equation_41 as rust_equation_41, equation_42 as rust_equation_42, 
    equation_43 as rust_equation_43, equation_44 as rust_equation_44
};

// Equation 28 module functions
#[pyfunction]
fn q_fo(a_t: f64, a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_28::q_fo(a_t, a_v, h_v))
}

#[pymodule]
fn equation_28(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_fo, m)?)?;
    Ok(())
}

// Equation 29 module functions
#[pyfunction]
#[pyo3(name = "q_fo")]
fn q_fo_29(h_k: f64, a_t: f64, a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_29::q_fo(h_k, a_t, a_v, h_v))
}

#[pymodule]
fn equation_29(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_fo_29, m)?)?;
    Ok(())
}

// Equation 33 module functions
#[pyfunction]
fn q_max_vc(a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_33::q_max_vc(a_v, h_v))
}

#[pymodule]
fn equation_33(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_max_vc, m)?)?;
    Ok(())
}

// Equation 4 module functions
#[pyfunction]
fn q_max_fc(a_f: f64, hrrpua: f64) -> PyResult<f64> {
    Ok(rust_equation_4::q_max_fc(a_f, hrrpua))
}

#[pymodule]
fn equation_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q_max_fc, m)?)?;
    Ok(())
}

// Equation 41 module functions
#[pyfunction]
fn calculate_41(omega: f64) -> PyResult<f64> {
    Ok(rust_equation_41::calculate(omega))
}

#[pymodule]
fn equation_41(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_41, m)?)?;
    Ok(())
}

// Equation 42 module functions
#[pyfunction]
fn calculate_42(a_t: f64, a_v: f64, h_v: f64) -> PyResult<f64> {
    Ok(rust_equation_42::calculate(a_t, a_v, h_v))
}

#[pymodule]
fn equation_42(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_42, m)?)?;
    Ok(())
}

// Equation 43 module functions
#[pyfunction]
fn calculate_43(t_g_max: f64, psi: f64) -> PyResult<f64> {
    Ok(rust_equation_43::calculate(t_g_max, psi))
}

#[pymodule]
fn equation_43(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_43, m)?)?;
    Ok(())
}

// Equation 44 module functions
#[pyfunction]
fn calculate_44(m_e: f64, a_v: f64, a_t: f64) -> PyResult<f64> {
    Ok(rust_equation_44::calculate(m_e, a_v, a_t))
}

#[pymodule]
fn equation_44(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_44, m)?)?;
    Ok(())
}

#[pymodule]
fn section_8(m: &Bound<'_, PyModule>) -> PyResult<()> {
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

#[pymodule]
fn part_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(section_8))?;
    Ok(())
}

#[pymodule]
fn pd_7974(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(part_1))?;
    Ok(())
}

#[pymodule]
fn openfire(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(pd_7974))?;
    Ok(())
}
