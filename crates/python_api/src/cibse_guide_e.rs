pub mod chapter_6;
pub mod chapter_7;
pub mod chapter_10;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn cibse_guide_e(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_6::chapter_6))?;
    m.add_wrapped(wrap_pymodule!(chapter_7::chapter_7))?;
    m.add_wrapped(wrap_pymodule!(chapter_10::chapter_10))?;
    Ok(())
}