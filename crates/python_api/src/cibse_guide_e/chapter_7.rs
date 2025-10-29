use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import CIBSE Guide E chapter 7 functions
use ::openfire::cibse_guide_e::chapter_7::{
    equation_7_2 as rust_equation_7_2, equation_7_3 as rust_equation_7_3,
    equation_7_6 as rust_equation_7_6, equation_7_7 as rust_equation_7_7,
    equation_7_8 as rust_equation_7_8, equation_7_9 as rust_equation_7_9,
};

// Equation 7_2 module functions
#[pyfunction]
fn stair_capacity(w: f64, n: i32) -> PyResult<i32> {
    Ok(rust_equation_7_2::stair_capacity(w, n))
}

#[pymodule]
fn equation_7_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stair_capacity, m)?)?;
    Ok(())
}

// Equation 7_3 module functions
#[pyfunction]
fn required_width_stair(p: i32, n: i32) -> PyResult<f64> {
    Ok(rust_equation_7_3::required_width_stair(p, n))
}

#[pymodule]
fn equation_7_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(required_width_stair, m)?)?;
    Ok(())
}

// Equation 7_6 module functions
#[pyfunction]
fn maximum_flowrate_persons(w: f64) -> PyResult<f64> {
    Ok(rust_equation_7_6::maximum_flowrate_persons(w))
}

#[pymodule]
fn equation_7_6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(maximum_flowrate_persons, m)?)?;
    Ok(())
}

// Equation 7_7 module functions
#[pyfunction]
fn maximum_people_in_stair(p: f64, a: f64, s: i32) -> PyResult<i32> {
    Ok(rust_equation_7_7::maximum_people_in_stair(p, a, s))
}

#[pymodule]
fn equation_7_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(maximum_people_in_stair, m)?)?;
    Ok(())
}

// Equation 7_8 module functions
#[pyfunction]
fn exit_capacity_stair(w_s: f64, t: f64, a: f64, s: i32) -> PyResult<i32> {
    Ok(rust_equation_7_8::exit_capacity_stair(w_s, t, a, s))
}

#[pymodule]
fn equation_7_8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(exit_capacity_stair, m)?)?;
    Ok(())
}

// Equation 7_9 module functions
#[pyfunction]
fn acceptance_capacity_stair(w_e: f64, t: f64, rho: f64, a: f64, s: i32) -> PyResult<i32> {
    Ok(rust_equation_7_9::acceptance_capacity_stair(w_e, t, rho, a, s))
}

#[pymodule]
fn equation_7_9(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(acceptance_capacity_stair, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_7_2))?;
    m.add_wrapped(wrap_pymodule!(equation_7_3))?;
    m.add_wrapped(wrap_pymodule!(equation_7_6))?;
    m.add_wrapped(wrap_pymodule!(equation_7_7))?;
    m.add_wrapped(wrap_pymodule!(equation_7_8))?;
    m.add_wrapped(wrap_pymodule!(equation_7_9))?;
    Ok(())
}