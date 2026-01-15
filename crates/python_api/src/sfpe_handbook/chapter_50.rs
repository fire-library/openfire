pub mod equation_50_1;
pub mod equation_50_14;
pub mod equation_50_15;
pub mod equation_50_16;
pub mod equation_50_18;
pub mod equation_50_19;
pub mod equation_50_2;
pub mod equation_50_20;
pub mod equation_50_4;
pub mod equation_50_6;
pub mod equation_50_7;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// This chapter contains equations for smoke control applications.
pub fn chapter_50(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_50_1::equation_50_1))?;
    m.add_wrapped(wrap_pymodule!(equation_50_2::equation_50_2))?;
    m.add_wrapped(wrap_pymodule!(equation_50_4::equation_50_4))?;
    m.add_wrapped(wrap_pymodule!(equation_50_6::equation_50_6))?;
    m.add_wrapped(wrap_pymodule!(equation_50_7::equation_50_7))?;
    m.add_wrapped(wrap_pymodule!(equation_50_14::equation_50_14))?;
    m.add_wrapped(wrap_pymodule!(equation_50_15::equation_50_15))?;
    m.add_wrapped(wrap_pymodule!(equation_50_16::equation_50_16))?;
    m.add_wrapped(wrap_pymodule!(equation_50_18::equation_50_18))?;
    m.add_wrapped(wrap_pymodule!(equation_50_19::equation_50_19))?;
    m.add_wrapped(wrap_pymodule!(equation_50_20::equation_50_20))?;
    Ok(())
}
