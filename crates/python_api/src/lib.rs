#![allow(unsafe_op_in_unsafe_fn)]

mod br_187;
mod bs9999;
mod cibse_guide_e;
mod fire_dynamics_tools;
mod introduction_to_fire_dynamics;
mod pd_7974;
mod sfpe_handbook;
mod tr_17;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
fn openfire(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(pd_7974::pd_7974))?;
    m.add_wrapped(wrap_pymodule!(br_187::br_187))?;
    m.add_wrapped(wrap_pymodule!(bs9999::bs9999))?;
    m.add_wrapped(wrap_pymodule!(cibse_guide_e::cibse_guide_e))?;
    m.add_wrapped(wrap_pymodule!(fire_dynamics_tools::fire_dynamics_tools))?;
    m.add_wrapped(wrap_pymodule!(
        introduction_to_fire_dynamics::introduction_to_fire_dynamics
    ))?;
    m.add_wrapped(wrap_pymodule!(sfpe_handbook::sfpe_handbook))?;
    m.add_wrapped(wrap_pymodule!(tr_17::tr_17))?;
    Ok(())
}
