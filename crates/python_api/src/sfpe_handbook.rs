pub mod chapter_14;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
/// SFPE Handbook of Fire Protection Engineering - Fire engineering correlations and calculations.
///
/// This module contains correlations, equations, and calculation methods from the
/// Society of Fire Protection Engineers (SFPE) Handbook of Fire Protection Engineering.
/// The handbook serves as a reference for fire protection engineering calculations
/// and methodologies.
pub fn sfpe_handbook(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_14::chapter_14))?;
    Ok(())
}
