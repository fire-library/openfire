pub mod section_8;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn part_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(section_8::section_8))?;
    Ok(())
}
