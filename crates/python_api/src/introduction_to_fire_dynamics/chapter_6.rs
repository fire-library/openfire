use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import introduction_to_fire_dynamics chapter 6 functions
use ::openfire::introduction_to_fire_dynamics::chapter_6::equation_6_32 as rust_equation_6_32;

// Equation 6_32 module functions
#[pyfunction]
fn time_to_ignition_thermally_thick(
    k: f64,
    rho: f64,
    c: f64,
    temp_ig: f64,
    temp_o: f64,
    q_r: f64,
) -> PyResult<f64> {
    Ok(rust_equation_6_32::time_to_ignition_thermally_thick(
        k, rho, c, temp_ig, temp_o, q_r,
    ))
}

#[pymodule]
fn equation_6_32(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(time_to_ignition_thermally_thick, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_6_32))?;
    Ok(())
}
