pub mod chapter_15;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn bs9999(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_15::chapter_15))?;
    Ok(())
}