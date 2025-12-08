use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use ::openfire::fire_dynamics_tools::chapter_4::equation_4_1 as rust_equation_4_1;
use ::openfire::fire_dynamics_tools::chapter_4::equation_4_2 as rust_equation_4_2;
use ::openfire::fire_dynamics_tools::chapter_4::equation_4_3 as rust_equation_4_3;

#[pyfunction]
/// Calculates wall fire flame height (Equation 4-1).
///
/// This equation determines the flame height for fires adjacent to walls based on
/// the heat release rate. The wall effect increases flame height compared to
/// free-burning fires due to reduced air entrainment.
///
/// .. math::
///
///    h_f = 0.034 \cdot q^{\frac{2}{3}}
///
/// where:
///
/// - :math:`h_f` is the flame height (m)
/// - :math:`q` is the heat release rate (kW)
///
/// Args:
///     q (float): Heat release rate (kW)
///
/// Returns:
///     float: Wall fire flame height (m)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_4.equation_4_1.wall_fire_flame_height(700.0)
///     >>> print(f"{result:.2f} m")
fn wall_fire_flame_height(q: f64) -> PyResult<f64> {
    Ok(rust_equation_4_1::wall_fire_flame_height(q))
}

#[pymodule]
/// This module contains calculations for determining flame height of fires
/// adjacent to walls. Wall fires exhibit different characteristics compared
/// to free-burning fires due to the restriction of air entrainment from one side.
fn equation_4_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wall_fire_flame_height, m)?)?;
    Ok(())
}

#[pyfunction]
/// Calculates line fire flame height (Equation 4-2).
///
/// This equation determines the flame height for line fires, which are fires
/// that spread along a linear fuel source. Line fires have different flame
/// characteristics compared to point source fires due to their geometry.
///
/// .. math::
///
///    h_f = 0.017 \cdot q^{\frac{2}{3}}
///
/// where:
///
/// - :math:`h_f` is the flame height (m)
/// - :math:`q` is the heat release rate (kW)
///
/// Args:
///     q (float): Heat release rate (kW)
///
/// Returns:
///     float: Line fire flame height (m)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_4.equation_4_2.line_fire_flame_height(700.0)
///     >>> print(f"{result:.2f} m")
fn line_fire_flame_height(q: f64) -> PyResult<f64> {
    Ok(rust_equation_4_2::line_fire_flame_height(q))
}

#[pymodule]
/// This module contains calculations for determining flame height of line fires.
/// Line fires occur when fuel is arranged in a linear pattern, creating different
/// flame dynamics compared to point source or area fires.
fn equation_4_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(line_fire_flame_height, m)?)?;
    Ok(())
}

#[pyfunction]
/// Calculates corner fire flame height (Equation 4-3).
///
/// This equation determines the flame height for fires located in corners,
/// where two walls meet. Corner fires have restricted air entrainment from
/// two sides, resulting in higher flame heights compared to wall fires.
///
/// .. math::
///
///    h_f = 0.075 \cdot q^{\frac{3}{5}}
///
/// where:
///
/// - :math:`h_f` is the flame height (m)
/// - :math:`q` is the heat release rate (kW)
///
/// Args:
///     q (float): Heat release rate (kW)
///
/// Returns:
///     float: Corner fire flame height (m)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_4.equation_4_3.corner_fire_flame_height(700.0)
///     >>> print(f"{result:.2f} m")
fn corner_fire_flame_height(q: f64) -> PyResult<f64> {
    Ok(rust_equation_4_3::corner_fire_flame_height(q))
}

#[pymodule]
/// This module contains calculations for determining flame height of fires
/// located in corners. Corner fires have air entrainment restricted from
/// two sides, leading to different flame behavior compared to wall or free fires.
fn equation_4_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(corner_fire_flame_height, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 4 - Fire Dynamics Tools.
///
/// This chapter contains equations for fire dynamics calculations including
/// flame height correlations for different fire configurations. These equations
/// are fundamental for fire safety engineering analysis and design.
pub fn chapter_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_4_1))?;
    m.add_wrapped(wrap_pymodule!(equation_4_2))?;
    m.add_wrapped(wrap_pymodule!(equation_4_3))?;
    Ok(())
}
