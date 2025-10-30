pub mod alpert;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn chapter_14(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(alpert::alpert))?;
    Ok(())
}
