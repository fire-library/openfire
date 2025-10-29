pub mod part_1;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn pd_7974(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(part_1::part_1))?;
    Ok(())
}