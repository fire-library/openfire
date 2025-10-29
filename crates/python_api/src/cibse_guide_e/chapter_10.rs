use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import CIBSE Guide E chapter 10 functions
use ::openfire::cibse_guide_e::chapter_10::{
    equation_10_1 as rust_equation_10_1, equation_10_10 as rust_equation_10_10,
    equation_10_11 as rust_equation_10_11, equation_10_12 as rust_equation_10_12,
    equation_10_2 as rust_equation_10_2, equation_10_3 as rust_equation_10_3,
    equation_10_4 as rust_equation_10_4, equation_10_7 as rust_equation_10_7,
    equation_10_8 as rust_equation_10_8,
};

// Equation 10_1 module functions
#[pyfunction]
fn max_volumetric_flow_rate(gamma: f64, d: f64, t_s: f64, t_0: f64) -> PyResult<f64> {
    Ok(rust_equation_10_1::max_volumetric_flow_rate(gamma, d, t_s, t_0))
}

#[pymodule]
fn equation_10_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(max_volumetric_flow_rate, m)?)?;
    Ok(())
}

// Equation 10_2 module functions
#[pyfunction]
fn min_separation_dist(v_e: f64) -> PyResult<f64> {
    Ok(rust_equation_10_2::min_separation_dist(v_e))
}

#[pymodule]
fn equation_10_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(min_separation_dist, m)?)?;
    Ok(())
}

// Equation 10_3 module functions
#[pyfunction]
fn volumetric_flow_rate(m: f64, t_s: f64, rho_0: f64, t_0: f64) -> PyResult<f64> {
    Ok(rust_equation_10_3::volumetric_flow_rate(m, t_s, rho_0, t_0))
}

#[pymodule]
fn equation_10_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(volumetric_flow_rate, m)?)?;
    Ok(())
}

// Equation 10_4 module functions
#[pyfunction]
fn time_burning_skin(q: f64) -> PyResult<f64> {
    Ok(rust_equation_10_4::time_burning_skin(q))
}

#[pymodule]
fn equation_10_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(time_burning_skin, m)?)?;
    Ok(())
}

// Equation 10_7 module functions
#[pyfunction]
fn visibility(k: f64, d: f64) -> PyResult<f64> {
    Ok(rust_equation_10_7::visibility(k, d))
}

#[pymodule]
fn equation_10_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(visibility, m)?)?;
    Ok(())
}

// Equation 10_8 module functions
#[pyfunction]
fn fractional_effective_dose(m_f: f64, t: f64, lc_50: f64) -> PyResult<f64> {
    Ok(rust_equation_10_8::fractional_effective_dose(m_f, t, lc_50))
}

#[pymodule]
fn equation_10_8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fractional_effective_dose, m)?)?;
    Ok(())
}

// Equation 10_10 module functions
#[pyfunction]
#[pyo3(name = "limiting_velocity")]
fn limiting_velocity_10_10(g: f64, h: f64, t_f: f64, t_0: f64) -> PyResult<f64> {
    Ok(rust_equation_10_10::limiting_velocity(g, h, t_f, t_0))
}

#[pymodule]
fn equation_10_10(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(limiting_velocity_10_10, m)?)?;
    Ok(())
}

// Equation 10_11 module functions
#[pyfunction]
#[pyo3(name = "limiting_velocity")]
fn limiting_velocity_10_11(q: f64, z: f64) -> PyResult<f64> {
    Ok(rust_equation_10_11::limiting_velocity(q, z))
}

#[pymodule]
fn equation_10_11(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(limiting_velocity_10_11, m)?)?;
    Ok(())
}

// Equation 10_12 module functions
#[pyfunction]
#[pyo3(name = "limiting_velocity")]
fn limiting_velocity_10_12(k: f64, g: f64, q: f64, omega: f64, rho: f64, c: f64, t: f64) -> PyResult<f64> {
    Ok(rust_equation_10_12::limiting_velocity(k, g, q, omega, rho, c, t))
}

#[pymodule]
fn equation_10_12(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(limiting_velocity_10_12, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_10(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_10_1))?;
    m.add_wrapped(wrap_pymodule!(equation_10_2))?;
    m.add_wrapped(wrap_pymodule!(equation_10_3))?;
    m.add_wrapped(wrap_pymodule!(equation_10_4))?;
    m.add_wrapped(wrap_pymodule!(equation_10_7))?;
    m.add_wrapped(wrap_pymodule!(equation_10_8))?;
    m.add_wrapped(wrap_pymodule!(equation_10_10))?;
    m.add_wrapped(wrap_pymodule!(equation_10_11))?;
    m.add_wrapped(wrap_pymodule!(equation_10_12))?;
    Ok(())
}