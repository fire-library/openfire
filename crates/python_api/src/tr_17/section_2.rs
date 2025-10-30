pub mod equation_1;

use pyo3::prelude::*;

#[pymodule]
pub fn section_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let equation_1_module = PyModule::new_bound(m.py(), "equation_1")?;
    equation_1::equation_1_intro(&equation_1_module)?;
    m.add_submodule(&equation_1_module)?;
    Ok(())
}
