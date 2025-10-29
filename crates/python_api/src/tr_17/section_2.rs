pub mod equation_1;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn section_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_1::equation_1))?;
    Ok(())
}
