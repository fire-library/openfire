use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import BR_187 appendix A functions
use ::openfire::br_187::appendix_a::{
    equation_a1 as rust_equation_a1, equation_a2 as rust_equation_a2,
    equation_a3 as rust_equation_a3, equation_a4 as rust_equation_a4,
    equation_a5 as rust_equation_a5,
};

// Equation A1 module functions
#[pyfunction]
fn radiation_intensity(sigma: f64, emissivity: f64, temperature: f64) -> PyResult<f64> {
    Ok(rust_equation_a1::radiation_intensity(
        sigma,
        emissivity,
        temperature,
    ))
}

#[pymodule]
fn equation_a1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(radiation_intensity, m)?)?;
    Ok(())
}

// Equation A2 module functions
#[pyfunction]
fn radiation_intensity_at_receiver(phi: f64, i_s: f64) -> PyResult<f64> {
    Ok(rust_equation_a2::radiation_intensity_at_receiver(phi, i_s))
}

#[pymodule]
fn equation_a2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(radiation_intensity_at_receiver, m)?)?;
    Ok(())
}

// Equation A3 module functions
#[pyfunction]
#[pyo3(name = "x")]
fn x_a3(w: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a3::x(w, s))
}

#[pyfunction]
#[pyo3(name = "y")]
fn y_a3(h: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a3::y(h, s))
}

#[pyfunction]
#[pyo3(name = "phi")]
fn phi_a3(x: f64, y: f64, additive: bool) -> PyResult<f64> {
    Ok(rust_equation_a3::phi(x, y, additive))
}

#[pymodule]
fn equation_a3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(x_a3, m)?)?;
    m.add_function(wrap_pyfunction!(y_a3, m)?)?;
    m.add_function(wrap_pyfunction!(phi_a3, m)?)?;
    Ok(())
}

// Equation A4 module functions
#[pyfunction]
#[pyo3(name = "x")]
fn x_a4(w: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a4::x(w, s))
}

#[pyfunction]
#[pyo3(name = "y")]
fn y_a4(h: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a4::y(h, s))
}

#[pyfunction]
#[pyo3(name = "phi")]
fn phi_a4(x: f64, y: f64, additive: bool) -> PyResult<f64> {
    Ok(rust_equation_a4::phi(x, y, additive))
}

#[pymodule]
fn equation_a4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(x_a4, m)?)?;
    m.add_function(wrap_pyfunction!(y_a4, m)?)?;
    m.add_function(wrap_pyfunction!(phi_a4, m)?)?;
    Ok(())
}

// Equation A5 module functions
#[pyfunction]
#[pyo3(name = "x")]
fn x_a5(w: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a5::x(w, s))
}

#[pyfunction]
#[pyo3(name = "y")]
fn y_a5(h: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a5::y(h, s))
}

#[pyfunction]
#[pyo3(name = "phi")]
fn phi_a5(x: f64, y: f64, additive: bool) -> PyResult<f64> {
    Ok(rust_equation_a5::phi(x, y, additive))
}

#[pymodule]
fn equation_a5(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(x_a5, m)?)?;
    m.add_function(wrap_pyfunction!(y_a5, m)?)?;
    m.add_function(wrap_pyfunction!(phi_a5, m)?)?;
    Ok(())
}

#[pymodule]
pub fn appendix_a(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_a1))?;
    m.add_wrapped(wrap_pymodule!(equation_a2))?;
    m.add_wrapped(wrap_pymodule!(equation_a3))?;
    m.add_wrapped(wrap_pymodule!(equation_a4))?;
    m.add_wrapped(wrap_pymodule!(equation_a5))?;
    Ok(())
}
