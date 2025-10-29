#![allow(unsafe_op_in_unsafe_fn)]

mod br_187;
mod bs9999;
mod cibse_guide_e;
mod fire_dynamics_tools;
mod pd_7974;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
fn openfire(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(pd_7974::pd_7974))?;
    m.add_wrapped(wrap_pymodule!(br_187::br_187))?;
    m.add_wrapped(wrap_pymodule!(bs9999::bs9999))?;
    m.add_wrapped(wrap_pymodule!(cibse_guide_e::cibse_guide_e))?;
    m.add_wrapped(wrap_pymodule!(fire_dynamics_tools::fire_dynamics_tools))?;
    Ok(())
}
