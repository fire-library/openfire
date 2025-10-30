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
fn heat_release_rate_flashover(a_vo: f64, h_o: f64) -> PyResult<f64> {
    Ok(rust_equation_6_7::heat_release_rate_flashover(a_vo, h_o))
}

#[pymodule]
fn equation_6_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(heat_release_rate_flashover, m)?)?;
    Ok(())
}

// Equation 6_55 module functions
#[pyfunction]
fn mean_flame_height(q_t: f64) -> PyResult<f64> {
    Ok(rust_equation_6_55::mean_flame_height(q_t))
}

#[pymodule]
fn equation_6_55(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean_flame_height, m)?)?;
    Ok(())
}

// Equation 6_57 module functions
#[pyfunction]
fn height_of_flame_aboveopening(r: f64, w: f64, h_o: f64) -> PyResult<f64> {
    Ok(rust_equation_6_57::height_of_flame_aboveopening(r, w, h_o))
}

#[pymodule]
fn equation_6_57(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(height_of_flame_aboveopening, m)?)?;
    Ok(())
}

// Equation 6_58 module functions
#[pyfunction]
fn vent_controlled_rate_of_burning(a_t: f64, a_o: f64, h_o: f64, w: f64, d: f64) -> PyResult<f64> {
    Ok(rust_equation_6_58::vent_controlled_rate_of_burning(
        a_t, a_o, h_o, w, d,
    ))
}

#[pymodule]
fn equation_6_58(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(vent_controlled_rate_of_burning, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_6_7))?;
    m.add_wrapped(wrap_pymodule!(equation_6_55))?;
    m.add_wrapped(wrap_pymodule!(equation_6_57))?;
    m.add_wrapped(wrap_pymodule!(equation_6_58))?;
    m.add_wrapped(wrap_pymodule!(appendix::appendix))?;
    Ok(())
}
