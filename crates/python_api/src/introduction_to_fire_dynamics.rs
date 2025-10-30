pub mod chapter_10;
pub mod chapter_6;

use pyo3::prelude::*;

#[pymodule]
pub fn introduction_to_fire_dynamics(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let chapter_6_module = PyModule::new_bound(m.py(), "chapter_6")?;
    chapter_6::chapter_6_intro(&chapter_6_module)?;
    m.add_submodule(&chapter_6_module)?;

    let chapter_10_module = PyModule::new_bound(m.py(), "chapter_10")?;
    chapter_10::chapter_10_intro(&chapter_10_module)?;
    m.add_submodule(&chapter_10_module)?;

    Ok(())
}
