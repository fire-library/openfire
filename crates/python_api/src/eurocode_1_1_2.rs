pub mod section_3;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn eurocode_1_1_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(section_3::section_3))?;
    Ok(())
}
