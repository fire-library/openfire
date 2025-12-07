use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import CIBSE Guide E chapter 10 functions
use ::openfire::cibse_guide_e::chapter_10::{
    equation_10_1 as rust_equation_10_1, equation_10_2 as rust_equation_10_2,
    equation_10_3 as rust_equation_10_3, equation_10_4 as rust_equation_10_4,
    equation_10_7 as rust_equation_10_7, equation_10_8 as rust_equation_10_8,
    equation_10_10 as rust_equation_10_10, equation_10_11 as rust_equation_10_11,
    equation_10_12 as rust_equation_10_12,
};

// Equation 10_1 module functions
#[pyfunction]
/// Calculates the maximum volumetric flow rate (Equation 10.1).
///
/// This equation determines the maximum volumetric flow rate through
/// an opening based on discharge coefficient, opening diameter, and temperature difference.
///
/// .. math::
///
///    V = 4.16 \cdot \gamma \cdot d^{5/2} \cdot \left(\frac{T_s - T_0}{T_0}\right)^{1/2}
///
/// where:
///
/// - :math:`V` is the maximum volumetric flow rate (m³/s)
/// - :math:`\gamma` is the discharge coefficient (dimensionless)
/// - :math:`d` is the diameter of opening (m)
/// - :math:`T_s` is the smoke temperature (K)
/// - :math:`T_0` is the ambient temperature (K)
///
/// Args:
///     gamma (float): Discharge coefficient (dimensionless)
///     d (float): Diameter of opening (m)
///     t_s (float): Smoke temperature (K)
///     t_0 (float): Ambient temperature (K)
///
/// Returns:
///     float: Maximum volumetric flow rate (m³/s)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_10.equation_10_1.max_volumetric_flow_rate(0.5, 1.5, 300.0, 290.0)
fn max_volumetric_flow_rate(gamma: f64, d: f64, t_s: f64, t_0: f64) -> PyResult<f64> {
    Ok(rust_equation_10_1::max_volumetric_flow_rate(
        gamma, d, t_s, t_0,
    ))
}

#[pymodule]
/// Equation 10.1 - Maximum Volumetric Flow Rate.
///
/// Calculates the maximum volumetric flow rate through an opening
/// based on temperature and geometric parameters.
fn equation_10_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(max_volumetric_flow_rate, m)?)?;
    Ok(())
}

// Equation 10_2 module functions
#[pyfunction]
/// Calculates the minimum separation distance between vents (Equation 10.2).
///
/// This equation determines the minimum separation distance
/// between exhaust vents based on the escape velocity.
///
/// .. math::
///
///    d = 0.9 \cdot V_e^{0.5}
///
/// where:
///
/// - :math:`d` is the minimum separation distance (m)
/// - :math:`V_e` is the escape velocity (m/s)
///
/// Args:
///     v_e (float): Escape velocity (m/s)
///
/// Returns:
///     float: Minimum separation distance (m)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_10.equation_10_2.min_separation_dist(0.3)
fn min_separation_dist(v_e: f64) -> PyResult<f64> {
    Ok(rust_equation_10_2::min_separation_dist(v_e))
}

#[pymodule]
/// Equation 10.2 - Minimum Separation Distance Between Vents.
///
/// Calculates the minimum separation distance based on escape velocity.
fn equation_10_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(min_separation_dist, m)?)?;
    Ok(())
}

// Equation 10_3 module functions
#[pyfunction]
/// Calculates the volumetric flow rate (Equation 10.3).
///
/// This equation determines the volumetric flow rate of smoke
/// based on mass flow rate, temperature, and density parameters.
///
/// .. math::
///
///    V = \frac{m \cdot T_s}{\rho_0 \cdot T_0}
///
/// where:
///
/// - :math:`V` is the volumetric flow rate (m³/s)
/// - :math:`m` is the mass flow rate of smoke exhaust (kg/s)
/// - :math:`T_s` is the absolute temperature of the smoke (K)
/// - :math:`\rho_0` is the density of air at ambient temperature (kg/m³)
/// - :math:`T_0` is the absolute ambient temperature (K)
///
/// Args:
///     m (float): Mass flow rate of smoke exhaust (kg/s)
///     t_s (float): Absolute temperature of the smoke (K)
///     rho_0 (float): Density of air at ambient temperature (kg/m³)
///     t_0 (float): Absolute ambient temperature (K)
///
/// Returns:
///     float: Volumetric flow rate (m³/s)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_10.equation_10_3.volumetric_flow_rate(2.0, 473.0, 1.2, 293.0)
fn volumetric_flow_rate(m: f64, t_s: f64, rho_0: f64, t_0: f64) -> PyResult<f64> {
    Ok(rust_equation_10_3::volumetric_flow_rate(m, t_s, rho_0, t_0))
}

#[pymodule]
/// Equation 10.3 - Volumetric Flow Rate.
///
/// Calculates the volumetric flow rate based on mass flow rate and temperature parameters.
fn equation_10_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(volumetric_flow_rate, m)?)?;
    Ok(())
}

// Equation 10_4 module functions
#[pyfunction]
/// Calculates the time to burning of skin due to radiant heat (Equation 10.4).
///
/// This equation determines the time time to burning of skin
/// due to radiant heat.
///
/// .. math::
///
///    t_{irad} = 1.33 \cdot q^{-1.35}
///
/// where:
///
/// - :math:`t` is the time to burning of skin (min)
/// - :math:`q` is the radiant heat flux (kW/m²)
///
/// Args:
///     q (float): Radiant heat flux (kW/m²)
///
/// Returns:
///     float: Time to burning of skin (min)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_10.equation_10_4.time_burning_skin(2.5)
fn time_burning_skin(q: f64) -> PyResult<f64> {
    Ok(rust_equation_10_4::time_burning_skin(q))
}

#[pymodule]
/// Equation 10.4 - Time to Burning Skin.
///
/// Calculates the time required for skin to reach a burning condition based on heat flux.
fn equation_10_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(time_burning_skin, m)?)?;
    Ok(())
}

// Equation 10_7 module functions
#[pyfunction]
/// visibility (furthest distance at which an object can be perceived) (Equation 10.7).
///
/// This equation determines the furthest distance at which an object can be perceived
/// based on the optical density per unit length and a visibility coefficient.
///
/// .. math::
///
///    S = \frac{D}{2.303 \cdot D}
///
/// where:
///
/// - :math:`S` is the furthest distance at which an object can be perceived (m)
/// - :math:`K` is the visibility coefficient (-)
/// - :math:`D` is the optical density per unit length (m^{-1})
///
/// Args:
///     K (float): Visibility coefficient (-)
///     D (float): Optical density per unit length (m^{-1})
///
/// Returns:
///     float: Furthest distance at which an object can be perceived (m)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_10.equation_10_7.visibility(8.0, 0.5)
fn visibility(k: f64, d: f64) -> PyResult<f64> {
    Ok(rust_equation_10_7::visibility(k, d))
}

#[pymodule]
/// Equation 10.7 - Visibility.
///
/// Calculates the optical density based on visibility distance and extinction coefficient.
fn equation_10_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(visibility, m)?)?;
    Ok(())
}

// Equation 10_8 module functions
#[pyfunction]
/// Calculates the fractional effective dose (Equation 10.8).
///
/// This equation determines the fractional effective dose for toxicity
/// assessment based on mass concentration, exposure time, and lethal exposure dose.
///
/// .. math::
///
///    FED = \frac{m_f \cdot t}{LC_{50}}
///
/// where:
///
/// - :math:`FED` is the fractional effective dose (dimensionless)
/// - :math:`m_f` is the mass concentration of fuel burned (g/m^{3})
/// - :math:`t` is the exposure time (min)
/// - :math:`LC_{50}` is the lethal exposure dose from the test subject for 50% mortality (g/m³·min)
///
/// Args:
///     m_f (float): Mass concentration of fuel burned (g/m^{3})
///     t (float): Exposure time (min)
///     lc_50 (float): Lethal exposure dose for 50% mortality (g/m³·min)
///
/// Returns:
///     float: Fractional effective dose (dimensionless)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_10.equation_10_8.fractional_effective_dose(2.0, 120.0, 1000.0)
fn fractional_effective_dose(m_f: f64, t: f64, lc_50: f64) -> PyResult<f64> {
    Ok(rust_equation_10_8::fractional_effective_dose(m_f, t, lc_50))
}

#[pymodule]
/// Equation 10.8 - Fractional Effective Dose.
///
/// Calculates the fractional effective dose for toxicity assessment.
fn equation_10_8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fractional_effective_dose, m)?)?;
    Ok(())
}

// Equation 10_10 module functions
#[pyfunction]
/// Calculates the limiting average air velocity for opposed air flow ventilation (Equation 10.10).
///
/// This equation determines the limiting average velocity for opposed air flow ventilation
/// towards the fire compartment, sufficient to prevent the outflow of smoke
///
/// .. math::
///
///    v_e = 0.64 \cdot \left(g \cdot H \cdot \frac{T_f - T_0}{T_f}\right)^{0.5}
///
/// where:
///
/// - :math:`v_e` is the limiting average air velocity (m/s)
/// - :math:`g` is the acceleration due to gravity (m/s²)
/// - :math:`h` is the height of the opening as measured from the bottom of the opening (m)
/// - :math:`T_f` is the temperature of the heated smoke (K)
/// - :math:`T_0` is the temperature of the ambient air (K)
///
/// Args:
///     g (float): Acceleration due to gravity (m/s²)
///     h (float): Height of the opening as measured from the bottom of the opening (m)
///     t_f (float): Temperature of the heated smoke (K)
///     t_0 (float): Temperature of the ambient air (K)
///
/// Returns:
///     float: Limiting average air velocity (m/s)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_10.equation_10_10.limiting_velocity(9.8, 2.2, 973.0, 293.0)
#[pyo3(name = "limiting_velocity")]
fn limiting_velocity_10_10(g: f64, h: f64, t_f: f64, t_0: f64) -> PyResult<f64> {
    Ok(rust_equation_10_10::limiting_velocity(g, h, t_f, t_0))
}

#[pymodule]
/// Equation 10.10 - Limiting Average Air Velocity for Opposed Air Flow Ventilation.
///
/// Calculates the limiting average air velocity based on buoyancy forces and temperature difference.
fn equation_10_10(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(limiting_velocity_10_10, m)?)?;
    Ok(())
}

// Equation 10_11 module functions
#[pyfunction]
/// Calculates the limiting velocity based on heat release rate (Equation 10.11).
///
/// This equation determines the limiting velocity for smoke control
/// based on heat release rate and height above fire source.
///
/// .. math::
///
///    v_e = 0.057 \cdot \left(\frac{q}{z}\right)^{1/3}
///
/// where:
///
/// - :math:`v_e` is the limiting velocity (m/s)
/// - :math:`q` is the heat release rate (kW)
/// - :math:`z` is the height above fire source (m)
///
/// Args:
///     q (float): Heat release rate (kW)
///     z (float): Height above fire source (m)
///
/// Returns:
///     float: Limiting velocity (m/s)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_10.equation_10_11.limiting_velocity(1000.0, 1.5)
#[pyo3(name = "limiting_velocity")]
fn limiting_velocity_10_11(q: f64, z: f64) -> PyResult<f64> {
    Ok(rust_equation_10_11::limiting_velocity(q, z))
}

#[pymodule]
/// Equation 10.11 - Limiting Velocity Based on Heat Release Rate.
///
/// Calculates the limiting velocity based on heat release rate and height above fire source.
fn equation_10_11(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(limiting_velocity_10_11, m)?)?;
    Ok(())
}

// Equation 10_12 module functions
#[pyfunction]
/// Calculates the limiting velocity with thermal properties (Equation 10.12).
///
/// This equation determines the limiting velocity for smoke control
/// incorporating thermal properties of air and environmental conditions.
///
/// .. math::
///
///    v_e = k \cdot \left(\frac{g \cdot q}{\omega \cdot \rho \cdot c \cdot T}\right)^{1/3}
///
/// where:
///
/// - :math:`v_e` is the limiting velocity (m/s)
/// - :math:`k` is a dimensionless coefficient (dimensionless)
/// - :math:`g` is the acceleration due to gravity (m/s²)
/// - :math:`q` is the heat release rate (kW)
/// - :math:`\omega` is the width parameter (m)
/// - :math:`\rho` is the air density (kg/m³)
/// - :math:`c` is the specific heat capacity (kJ/kg·K)
/// - :math:`T` is the temperature (K)
///
/// Args:
///     k (float): Dimensionless coefficient (dimensionless)
///     g (float): Acceleration due to gravity (m/s²)
///     q (float): Heat release rate (kW)
///     omega (float): Width parameter (m)
///     rho (float): Air density (kg/m³)
///     c (float): Specific heat capacity (kJ/kg·K)
///     t (float): Temperature (K)
///
/// Returns:
///     float: Limiting velocity (m/s)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_10.equation_10_12.limiting_velocity(1.0, 9.8, 1000.0, 2.5, 1.2, 1.0, 773.0)
#[pyo3(name = "limiting_velocity")]
fn limiting_velocity_10_12(
    k: f64,
    g: f64,
    q: f64,
    omega: f64,
    rho: f64,
    c: f64,
    t: f64,
) -> PyResult<f64> {
    Ok(rust_equation_10_12::limiting_velocity(
        k, g, q, omega, rho, c, t,
    ))
}

#[pymodule]
/// Equation 10.12 - Limiting Velocity with Thermal Properties.
///
/// Calculates the limiting velocity incorporating thermal properties and environmental conditions.
fn equation_10_12(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(limiting_velocity_10_12, m)?)?;
    Ok(())
}

#[pymodule]
pub fn chapter_10(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_10_1))?;
    m.add_wrapped(wrap_pymodule!(equation_10_2))?;
    m.add_wrapped(wrap_pymodule!(equation_10_3))?;
    m.add_wrapped(wrap_pymodule!(equation_10_4))?;
    m.add_wrapped(wrap_pymodule!(equation_10_7))?;
    m.add_wrapped(wrap_pymodule!(equation_10_8))?;
    m.add_wrapped(wrap_pymodule!(equation_10_10))?;
    m.add_wrapped(wrap_pymodule!(equation_10_11))?;
    m.add_wrapped(wrap_pymodule!(equation_10_12))?;
    Ok(())
}
