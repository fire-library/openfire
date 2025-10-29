pub mod section_2;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn tr_17(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(section_2::section_2))?;
    Ok(())
}
