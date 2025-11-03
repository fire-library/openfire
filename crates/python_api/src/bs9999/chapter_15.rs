use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import BS9999 chapter 15 functions
use ::openfire::bs9999::chapter_15::{
    figure_6a as rust_figure_6a, figure_6b as rust_figure_6b, figure_6c as rust_figure_6c,
};

// Figure 6a module functions
#[pyfunction]
#[pyo3(name = "calculate_exit_width")]
/// Calculate exit width for stairs serving upper floors only (BS9999 Chapter 15, Figure 6a).
///
/// This function calculates the required exit width for stairs that only serve floors above
/// the ground level, based on BS9999 fire safety calculations.
///
/// .. math::
///
///    W_{fe} = \begin{cases}
///    S_{up} + W_{se} & \text{if } n > 60 \text{ and } d < 2.0 \\
///    n \cdot x + 0.75 \cdot S_{up} & \text{otherwise}
///    \end{cases}
///
/// where:
///
/// - :math:`W_{fe}` is the required exit width (m)
/// - :math:`S_{up}` is the total floor area served by the stair on upper floors (m²)
/// - :math:`W_{se}` is the width of exit at discharge level (m)
/// - :math:`n` is the number of floors served by the stair above discharge level (floors)
/// - :math:`d` is the design occupant density (persons/m²)
/// - :math:`x` is the stair flow capacity (persons/m/min)
///
/// Args:
///     s_up (float): Total floor area served by the stair on upper floors (m²)
///     w_se (float): Width of exit at discharge level (m)
///     n (float): Number of floors served by the stair above discharge level (floors)
///     d (float): Design occupant density (persons/m²)
///     x (float): Stair flow capacity (persons/m/min)
///
/// Returns:
///     float: Required exit width (m)
///
/// Example:
///     >>> import ofire
///     >>> width = ofire.bs9999.chapter_15.figure_6a.calculate_exit_width(1000.0, 2.0, 5, 0.5, 40.0)
///     >>> print(f"Exit width: {width} m")
fn calculate_exit_width_6a(s_up: f64, w_se: f64, n: f64, d: f64, x: f64) -> PyResult<f64> {
    Ok(rust_figure_6a::calculate_exit_width(s_up, w_se, n, d, x))
}

#[pymodule]
fn figure_6a(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_exit_width_6a, m)?)?;
    Ok(())
}

// Figure 6b module functions
#[pyfunction]
#[pyo3(name = "calculate_exit_width")]
/// Calculate exit width for stairs serving upper and lower floors (BS9999 Chapter 15, Figure 6b).
///
/// This function calculates the required exit width for stairs that serve both floors above
/// and below the ground level, based on BS9999 fire safety calculations.
///
/// .. math::
///
///    W_{fe} = \begin{cases}
///    S_{up} + S_{dn} & \text{if } b > 60 \text{ and } d < 2.0 \\
///    b \cdot x + 0.75 \cdot S_{up} & \text{otherwise}
///    \end{cases}
///
/// where:
///
/// - :math:`W_{fe}` is the required exit width (m)
/// - :math:`S_{up}` is the total floor area served by the stair on upper floors (m²)
/// - :math:`S_{dn}` is the total floor area served by the stair on basement floors (m²)
/// - :math:`b` is the number of basement floors served by the stair (floors)
/// - :math:`d` is the design occupant density (persons/m²)
/// - :math:`x` is the stair flow capacity (persons/m/min)
///
/// Args:
///     b (float): Number of basement floors served by the stair (floors)
///     d (float): Design occupant density (persons/m²)
///     s_up (float): Total floor area served by the stair on upper floors (m²)
///     s_dn (float): Total floor area served by the stair on basement floors (m²)
///     x (float): Stair flow capacity (persons/m/min)
///
/// Returns:
///     float: Required exit width (m)
///
/// Example:
///     >>> import ofire
///     >>> width = ofire.bs9999.chapter_15.figure_6b.calculate_exit_width(2.0, 0.5, 1000.0, 500.0, 40.0)
///     >>> print(f"Exit width: {width} m")
fn calculate_exit_width_6b(b: f64, d: f64, s_up: f64, s_dn: f64, x: f64) -> PyResult<f64> {
    Ok(rust_figure_6b::calculate_exit_width(b, d, s_up, s_dn, x))
}

#[pymodule]
fn figure_6b(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_exit_width_6b, m)?)?;
    Ok(())
}

// Figure 6c module functions
#[pyfunction]
#[pyo3(name = "calculate_exit_width")]
/// Calculate exit width for complex stair configurations (BS9999 Chapter 15, Figure 6c).
///
/// This function calculates the required exit width for stairs with complex configurations
/// serving multiple floors above and below ground level, including considerations for
/// exit width at discharge level, based on BS9999 fire safety calculations.
///
/// .. math::
///
///    W_{fe} = \begin{cases}
///    S_{up} + S_{dn} + W_{se} & \text{if } (b + n) > 60 \text{ and } d < 2.0 \\
///    b \cdot x + n \cdot x + 0.75 \cdot S_{up} & \text{otherwise}
///    \end{cases}
///
/// where:
///
/// - :math:`W_{fe}` is the required exit width (m)
/// - :math:`S_{up}` is the total floor area served by the stair on upper floors (m²)
/// - :math:`S_{dn}` is the total floor area served by the stair on basement floors (m²)
/// - :math:`W_{se}` is the width of exit at discharge level (m)
/// - :math:`b` is the number of basement floors served by the stair (floors)
/// - :math:`n` is the number of floors served by the stair above discharge level (floors)
/// - :math:`d` is the design occupant density (persons/m²)
/// - :math:`x` is the stair flow capacity (persons/m/min)
///
/// Args:
///     b (float): Number of basement floors served by the stair (floors)
///     n (float): Number of floors served by the stair above discharge level (floors)
///     d (float): Design occupant density (persons/m²)
///     s_up (float): Total floor area served by the stair on upper floors (m²)
///     s_dn (float): Total floor area served by the stair on basement floors (m²)
///     w_se (float): Width of exit at discharge level (m)
///     x (float): Stair flow capacity (persons/m/min)
///
/// Returns:
///     float: Required exit width (m)
///
/// Example:
///     >>> import ofire
///     >>> width = ofire.bs9999.chapter_15.figure_6c.calculate_exit_width(2.0, 5.0, 0.5, 1000.0, 500.0, 2.0, 40.0)
///     >>> print(f"Exit width: {width} m")
fn calculate_exit_width_6c(
    b: f64,
    n: f64,
    d: f64,
    s_up: f64,
    s_dn: f64,
    w_se: f64,
    x: f64,
) -> PyResult<f64> {
    Ok(rust_figure_6c::calculate_exit_width(
        b, n, d, s_up, s_dn, w_se, x,
    ))
}

#[pymodule]
fn figure_6c(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_exit_width_6c, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_15(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(figure_6a))?;
    m.add_wrapped(wrap_pymodule!(figure_6b))?;
    m.add_wrapped(wrap_pymodule!(figure_6c))?;
    Ok(())
}
