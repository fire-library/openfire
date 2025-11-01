use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import BR_187 appendix A functions
use ::openfire::br_187::appendix_a::{
    equation_a1 as rust_equation_a1, equation_a2 as rust_equation_a2,
    equation_a3 as rust_equation_a3, equation_a4 as rust_equation_a4,
    equation_a5 as rust_equation_a5,
};

// Equation A1 module functions
#[pyfunction]
/// Calculate radiation intensity from a fire source (Equation A1).
/// 
/// The radiation intensity is calculated using the Stefan-Boltzmann law:
/// 
/// .. math::
/// 
///    I = \sigma \varepsilon T^4
/// 
/// where:
/// - :math:`I` is the radiation intensity (kW/m²)
/// - :math:`\sigma` is the Stefan-Boltzmann constant (5.67 × 10⁻¹¹ kW/m²K⁴)
/// - :math:`\varepsilon` is the surface emissivity (dimensionless, 0-1)
/// - :math:`T` is the absolute temperature (K)
/// 
/// Args:
///     sigma: Stefan-Boltzmann constant (kW/m²K⁴)
///     emissivity: Surface emissivity (dimensionless, 0-1)
///     temperature: Absolute temperature (K)
/// 
/// Returns:
///     Radiation intensity (kW/m²)
fn radiation_intensity(sigma: f64, emissivity: f64, temperature: f64) -> PyResult<f64> {
    Ok(rust_equation_a1::radiation_intensity(
        sigma,
        emissivity,
        temperature,
    ))
}

#[pymodule]
/// BR 187 Appendix A Equation A1 - Radiation intensity calculations.
/// 
/// Calculates thermal radiation intensity from fire sources using
/// the Stefan-Boltzmann law.
fn equation_a1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(radiation_intensity, m)?)?;
    Ok(())
}

// Equation A2 module functions
#[pyfunction]
/// Calculate radiation intensity at receiver location (Equation A2).
/// 
/// Calculates the thermal radiation intensity received at a target location
/// considering geometric view factors.
/// 
/// Args:
///     phi: View factor (dimensionless)
///     i_s: Source radiation intensity (W/m²)
/// 
/// Returns:
///     Radiation intensity at receiver (W/m²)
/// 
/// Example:
///     >>> import ofire
///     >>> received = ofire.br_187.appendix_a.equation_a2.radiation_intensity_at_receiver(0.15, 50000.0)
fn radiation_intensity_at_receiver(phi: f64, i_s: f64) -> PyResult<f64> {
    Ok(rust_equation_a2::radiation_intensity_at_receiver(phi, i_s))
}

#[pymodule]
/// BR 187 Appendix A Equation A2 - Radiation intensity at receiver calculations.
/// 
/// Calculates the thermal radiation intensity received at a target location
/// considering geometric view factors and source intensity.
fn equation_a2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(radiation_intensity_at_receiver, m)?)?;
    Ok(())
}

// Equation A3 module functions
#[pyfunction]
#[pyo3(name = "x")]
/// Calculate dimensionless width parameter (Equation A3).
/// 
/// Args:
///     w: Width of radiation source (m)
///     s: Distance from source to receiver (m)
/// 
/// Returns:
///     Dimensionless width parameter
fn x_a3(w: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a3::x(w, s))
}

#[pyfunction]
#[pyo3(name = "y")]
/// Calculate dimensionless height parameter (Equation A3).
/// 
/// Args:
///     h: Height of radiation source (m)
///     s: Distance from source to receiver (m)
/// 
/// Returns:
///     Dimensionless height parameter
fn y_a3(h: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a3::y(h, s))
}

#[pyfunction]
#[pyo3(name = "phi")]
/// Calculate view factor using dimensionless parameters (Equation A3).
/// 
/// Calculates the view factor using the formula:
/// 
/// .. math::
/// 
///    \phi = \frac{1}{\pi} \left[ \tan^{-1}\left(\frac{X}{\sqrt{Y^2 + 1}}\right) + \tan^{-1}\left(\frac{Y}{\sqrt{X^2 + 1}}\right) \right]
/// 
/// where:
/// - :math:`\phi` is the view factor (dimensionless)
/// - :math:`X` is the dimensionless width parameter
/// - :math:`Y` is the dimensionless height parameter
/// 
/// Args:
///     x: Dimensionless width parameter
///     y: Dimensionless height parameter
///     additive: Whether this view factor is positive or negative
/// 
/// Returns:
///     View factor (dimensionless)
fn phi_a3(x: f64, y: f64, additive: bool) -> PyResult<f64> {
    Ok(rust_equation_a3::phi(x, y, additive))
}

#[pymodule]
/// BR 187 Appendix A Equation A3 - Parallel source and receiver centre aligned.
/// 
/// Provides view factor calculations for parallel source and receiver surfaces
/// that are centre aligned.
fn equation_a3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(x_a3, m)?)?;
    m.add_function(wrap_pyfunction!(y_a3, m)?)?;
    m.add_function(wrap_pyfunction!(phi_a3, m)?)?;
    Ok(())
}

// Equation A4 module functions
#[pyfunction]
#[pyo3(name = "x")]
/// Calculate dimensionless width parameter (Equation A4).
/// 
/// Args:
///     w: Width of radiation source (m)
///     s: Distance from source to receiver (m)
/// 
/// Returns:
///     Dimensionless width parameter
fn x_a4(w: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a4::x(w, s))
}

#[pyfunction]
#[pyo3(name = "y")]
/// Calculate dimensionless height parameter (Equation A4).
/// 
/// Args:
///     h: Height of radiation source (m)
///     s: Distance from source to receiver (m)
/// 
/// Returns:
///     Dimensionless height parameter
fn y_a4(h: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a4::y(h, s))
}

#[pyfunction]
#[pyo3(name = "phi")]
/// Calculate view factor using alternative method (Equation A4).
/// 
/// Args:
///     x: Dimensionless width parameter
///     y: Dimensionless height parameter
///     additive: Whether this view factor is positive or negative
/// 
/// Returns:
///     View factor (dimensionless)
fn phi_a4(x: f64, y: f64, additive: bool) -> PyResult<f64> {
    Ok(rust_equation_a4::phi(x, y, additive))
}

#[pymodule]
/// BR 187 Appendix A Equation A4 - Parallel source and receiver corner aligned.
/// 
/// Provides view factor calculations for parallel source and receiver surfaces
/// that are corner aligned.
fn equation_a4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(x_a4, m)?)?;
    m.add_function(wrap_pyfunction!(y_a4, m)?)?;
    m.add_function(wrap_pyfunction!(phi_a4, m)?)?;
    Ok(())
}

// Equation A5 module functions
#[pyfunction]
#[pyo3(name = "x")]
/// Calculate dimensionless width parameter (Equation A5).
/// 
/// Args:
///     w: Width of radiation source (m)
///     s: Distance from source to receiver (m)
/// 
/// Returns:
///     Dimensionless width parameter
fn x_a5(w: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a5::x(w, s))
}

#[pyfunction]
#[pyo3(name = "y")]
/// Calculate dimensionless height parameter (Equation A5).
/// 
/// Args:
///     h: Height of radiation source (m)
///     s: Distance from source to receiver (m)
/// 
/// Returns:
///     Dimensionless height parameter
fn y_a5(h: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a5::y(h, s))
}

#[pyfunction]
#[pyo3(name = "phi")]
/// Calculate view factor for specific geometric configuration (Equation A5).
/// 
/// Args:
///     x: Dimensionless width parameter
///     y: Dimensionless height parameter
///     additive: Whether this view factor is positive or negative
/// 
/// Returns:
///     View factor (dimensionless)
fn phi_a5(x: f64, y: f64, additive: bool) -> PyResult<f64> {
    Ok(rust_equation_a5::phi(x, y, additive))
}

#[pymodule]
/// BR 187 Appendix A Equation A5 - Perpendicular source and receiver corner aligned.
/// 
/// Provides view factor calculations for perpendicular source and receiver surfaces
/// that are corner aligned.
fn equation_a5(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(x_a5, m)?)?;
    m.add_function(wrap_pyfunction!(y_a5, m)?)?;
    m.add_function(wrap_pyfunction!(phi_a5, m)?)?;
    Ok(())
}

#[pymodule]
/// BR 187 Appendix A - Thermal radiation calculations.
/// 
/// This appendix provides comprehensive calculations for thermal radiation
/// from fire sources, including radiation intensity calculations and view
/// factor determinations for various geometric configurations.
/// 
/// Available equations:
///     equation_a1: Radiation intensity from fire sources (Stefan-Boltzmann law)
///     equation_a2: Radiation intensity at receiver locations
///     equation_a3: Parallel source and receiver centre aligned
///     equation_a4: Parallel source and receiver corner aligned
///     equation_a5: Perpendicular source and receiver corner aligned
/// 
/// These calculations are essential for assessing thermal radiation exposure
/// in external fire spread scenarios between buildings.
pub fn appendix_a(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_a1))?;
    m.add_wrapped(wrap_pymodule!(equation_a2))?;
    m.add_wrapped(wrap_pymodule!(equation_a3))?;
    m.add_wrapped(wrap_pymodule!(equation_a4))?;
    m.add_wrapped(wrap_pymodule!(equation_a5))?;
    Ok(())
}
