pub mod chapter_2;
pub mod chapter_4;
pub mod chapter_5;
pub mod chapter_9;
pub mod chapter_18;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn fire_dynamics_tools(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_2::chapter_2))?;
    m.add_wrapped(wrap_pymodule!(chapter_4::chapter_4))?;
    m.add_wrapped(wrap_pymodule!(chapter_5::chapter_5))?;
    m.add_wrapped(wrap_pymodule!(chapter_9::chapter_9))?;
    m.add_wrapped(wrap_pymodule!(chapter_18::chapter_18))?;
    Ok(())
}
