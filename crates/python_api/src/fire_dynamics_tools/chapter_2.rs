use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import fire_dynamics_tools chapter 2 functions
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_10 as rust_equation_2_10;

// Equation 2_10 module functions
#[pyfunction]
fn height_smoke_layer_interface(
    k: f64,
    q: f64,
    t: Vec<f64>,
    a_c: f64,
    h_c: f64,
) -> PyResult<Vec<f64>> {
    Ok(rust_equation_2_10::height_smoke_layer_interface(
        k, q, t, a_c, h_c,
    ))
}

#[pymodule]
fn equation_2_10(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(height_smoke_layer_interface, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_2_10))?;
    Ok(())
}
