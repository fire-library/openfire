use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use openfire::sfpe_handbook::chapter_50::equation_50_14 as rust_equation_50_14;

#[pyfunction]
/// This function calculates the door opening force resulting from the pressure differences produced by smoke control systems.
///
/// .. math::
///
///    F_{d} = F_{dc} + \frac{w \times a \times \Delta p}{2 \times (w - d)}
///
/// where:
///
/// - :math:`F` is the total door opening force (N)
/// - :math:`F_{dc}` is the door closer force (N)
/// - :math:`W` is the door width (m)
/// - :math:`A` is the door area (m)
/// - :math:`\Delta p` is the pressure difference across the door (Pa)
/// - :math:`d` is the distance from door knob to knob side of the door (m)
///
/// Args:
///     f_dc (float): Door closer force (N)
///     W (float): Door width (m)
///     A (float): Door area (m^2)
///     delta_p (float): Pressure difference across the door (Pa)
///     d (float): Distance from door knob to knob side of the door (m)
///
/// Returns:
///     float: Door opening force (N)
///
/// Assumptions:
///     None stated.
///
/// Limitations:
///     None stated.
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.sfpe_handbook.chapter_50.equation_50_14.door_opening_force(40.0, 0.9, 1.9, 25.0, 0.05)
///     >>> print(f"{result:.6f} N")
fn door_opening_force(f_dc: f64, w: f64, a: f64, delta_p: f64, d: f64) -> PyResult<f64> {
    Ok(rust_equation_50_14::door_opening_force(
        f_dc, w, a, delta_p, d,
    ))
}

#[pymodule]
pub fn equation_50_14(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(door_opening_force, m)?)?;
    Ok(())
}
