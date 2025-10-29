use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import BS9999 chapter 15 functions
use ::openfire::bs9999::chapter_15::{
    figure_6a as rust_figure_6a, figure_6b as rust_figure_6b, figure_6c as rust_figure_6c,
};

// Figure 6a module functions
#[pyfunction]
#[pyo3(name = "calculate_exit_width")]
fn calculate_exit_width_6a(s_up: f64, w_se: f64, n: f64, d: f64, x: f64) -> PyResult<f64> {
    Ok(rust_figure_6a::calculate_exit_width(s_up, w_se, n, d, x))
}

#[pymodule]
fn figure_6a(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_exit_width_6a, m)?)?;
    Ok(())
}

// Figure 6b module functions
#[pyfunction]
#[pyo3(name = "calculate_exit_width")]
fn calculate_exit_width_6b(b: f64, d: f64, s_up: f64, s_dn: f64, x: f64) -> PyResult<f64> {
    Ok(rust_figure_6b::calculate_exit_width(b, d, s_up, s_dn, x))
}

#[pymodule]
fn figure_6b(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_exit_width_6b, m)?)?;
    Ok(())
}

// Figure 6c module functions
#[pyfunction]
#[pyo3(name = "calculate_exit_width")]
fn calculate_exit_width_6c(
    b: f64,
    n: f64,
    d: f64,
    s_up: f64,
    s_dn: f64,
    w_se: f64,
    x: f64,
) -> PyResult<f64> {
    Ok(rust_figure_6c::calculate_exit_width(
        b, n, d, s_up, s_dn, w_se, x,
    ))
}

#[pymodule]
fn figure_6c(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_exit_width_6c, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_15(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(figure_6a))?;
    m.add_wrapped(wrap_pymodule!(figure_6b))?;
    m.add_wrapped(wrap_pymodule!(figure_6c))?;
    Ok(())
}
