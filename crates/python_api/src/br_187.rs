pub mod appendix_a;
pub mod chapter_1;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn br_187(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(appendix_a::appendix_a))?;
    m.add_wrapped(wrap_pymodule!(chapter_1::chapter_1))?;
    Ok(())
}
