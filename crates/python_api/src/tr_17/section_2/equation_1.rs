use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import tr_17 section 2 equation 1 functions
use ::openfire::tr17::section_2::equation_1 as rust_equation_1;

#[pyfunction]
fn calculate_nondime_hrr(
    q_dot: f64,
    rho_a: f64,
    c_p: f64,
    t_a: f64,
    g: f64,
    h_e: f64,
) -> PyResult<f64> {
    Ok(rust_equation_1::calculate_nondime_hrr(
        q_dot, rho_a, c_p, t_a, g, h_e,
    ))
}

pub fn equation_1_intro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_nondime_hrr, m)?)?;
    Ok(())
}
