pub mod chapter_2;
pub mod chapter_4;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn fire_dynamics_tools(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_2::chapter_2))?;
    m.add_wrapped(wrap_pymodule!(chapter_4::chapter_4))?;
    Ok(())
}
