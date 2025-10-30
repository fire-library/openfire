use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import sfpe_handbook chapter 14 alpert heat_release functions
use ::openfire::sfpe_handbook::chapter_14::alpert::heat_release as rust_heat_release;

#[pyfunction]
fn from_temperature_and_position(
    temp: f64,
    temp_amb: f64,
    height: f64,
    radial_position: f64,
) -> PyResult<f64> {
    Ok(rust_heat_release::from_temperature_and_position(
        temp,
        temp_amb,
        height,
        radial_position,
    ))
}

#[pymodule]
pub fn heat_release(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_temperature_and_position, m)?)?;
    Ok(())
}
