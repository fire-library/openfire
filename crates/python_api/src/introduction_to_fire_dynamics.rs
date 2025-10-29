pub mod chapter_10;
pub mod chapter_6;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn introduction_to_fire_dynamics(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_6::chapter_6))?;
    m.add_wrapped(wrap_pymodule!(chapter_10::chapter_10))?;
    Ok(())
}
