pub mod equation_1;

use pyo3::prelude::*;

#[pymodule]
/// Section 2 - Non-dimensional parameters and scaling relationships.
///
/// This section contains equations and calculations for non-dimensional
/// parameters used in fire dynamics analysis and scaling relationships
/// for fire behavior correlation.
pub fn section_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let equation_1_module = PyModule::new_bound(m.py(), "equation_1")?;
    equation_1::equation_1_intro(&equation_1_module)?;
    m.add_submodule(&equation_1_module)?;
    Ok(())
}
