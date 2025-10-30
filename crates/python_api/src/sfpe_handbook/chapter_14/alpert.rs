pub mod heat_release;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn alpert(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(heat_release::heat_release))?;
    Ok(())
}
