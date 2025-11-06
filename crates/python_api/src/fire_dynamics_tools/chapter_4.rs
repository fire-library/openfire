use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use ::openfire::fire_dynamics_tools::chapter_4::equation_4_1 as rust_equation_4_1;
use ::openfire::fire_dynamics_tools::chapter_4::equation_4_3 as rust_equation_4_3;

#[pyfunction]
fn wall_fire_flame_height(q: f64) -> PyResult<f64> {
    Ok(rust_equation_4_1::wall_fire_flame_height(q))
}

#[pyfunction]
fn wall_fire_flame_height_equation(h_f: String, q: String) -> PyResult<String> {
    Ok(rust_equation_4_1::wall_fire_flame_height_equation(h_f, q))
}

#[pymodule]
fn equation_4_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wall_fire_flame_height, m)?)?;
    m.add_function(wrap_pyfunction!(wall_fire_flame_height_equation, m)?)?;
    Ok(())
}

#[pyfunction]
fn corner_fire_flame_height(q: f64) -> PyResult<f64> {
    Ok(rust_equation_4_3::corner_fire_flame_height(q))
}

#[pyfunction]
fn corner_fire_flame_height_equation(h_f: String, q: String) -> PyResult<String> {
    Ok(rust_equation_4_3::corner_fire_flame_height_equation(h_f, q))
}

#[pymodule]
fn equation_4_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(corner_fire_flame_height, m)?)?;
    m.add_function(wrap_pyfunction!(corner_fire_flame_height_equation, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_4_1))?;
    m.add_wrapped(wrap_pymodule!(equation_4_3))?;
    Ok(())
}