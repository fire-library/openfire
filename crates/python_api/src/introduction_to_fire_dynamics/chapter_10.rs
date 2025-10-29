use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import introduction_to_fire_dynamics chapter 10 functions
use ::openfire::introduction_to_fire_dynamics::chapter_10::equation_10_18 as rust_equation_10_18;

// Equation 10_18 module functions
#[pyfunction]
fn calculate(rho: f64, g: f64, a_w: f64, h: f64, a_f: f64) -> PyResult<f64> {
    Ok(rust_equation_10_18::calculate(rho, g, a_w, h, a_f))
}

#[pyfunction]
fn heating_regime(number: f64) -> PyResult<String> {
    let regime = rust_equation_10_18::heating_regime(number);
    Ok(regime.to_string())
}

#[pymodule]
fn equation_10_18(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;
    m.add_function(wrap_pyfunction!(heating_regime, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_10(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_10_18))?;
    Ok(())
}
