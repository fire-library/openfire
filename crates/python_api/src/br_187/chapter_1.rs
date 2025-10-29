use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import BR_187 chapter 1 functions
use ::openfire::br_187::chapter_1::equation_1 as rust_equation_1;

// Equation 1 module functions
#[pyfunction]
fn calculate_ventilation_factor(a_s: f64, a: f64, h: f64) -> PyResult<f64> {
    Ok(rust_equation_1::calculate_ventilation_factor(a_s, a, h))
}

#[pymodule]
fn equation_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_ventilation_factor, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_1))?;
    Ok(())
}
