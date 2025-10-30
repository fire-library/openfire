pub mod chapter_14;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn sfpe_handbook(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_14::chapter_14))?;
    Ok(())
}
