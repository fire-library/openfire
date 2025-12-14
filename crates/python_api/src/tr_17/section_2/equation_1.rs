use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import tr_17 section 2 equation 1 functions
use openfire::tr17::section_2::{equation_1 as rust_equation_1};

#[pyfunction]
/// Calculate non-dimensional heat release rate (Equation 1).
///
/// This function calculates the non-dimensional heat release rate parameter
/// used in fire dynamics calculations. This non-dimensional parameter is
/// useful for scaling fire behavior and correlating experimental data.
///
/// .. math::
///    :nowrap:
///
///    \begin{align}
///    \dot{Q}^* = \frac{\dot{Q}}{\rho_{\infty} c_p T_{\infty} \sqrt{g} H_e^{5/2}}
///    \end{align}
///
/// where:
///
/// - :math:`\dot{Q}^*` is the non-dimensional heat release rate (dimensionless)
/// - :math:`\dot{Q}` is the heat release rate (kW)
/// - :math:`\rho_{\infty}` is the ambient air density (kg/m³)
/// - :math:`c_p` is the specific heat capacity of air (kJ/kg·K)
/// - :math:`T_{\infty}` is the ambient temperature (K)
/// - :math:`g` is the gravitational acceleration (m/s²)
/// - :math:`H_e` is the characteristic height (m)
///
/// Args:
///     q_dot (float): Heat release rate (kW)
///     rho_a (float): Ambient air density (kg/m³)
///     c_p (float): Specific heat capacity of air (kJ/kg·K)
///     t_a (float): Ambient temperature (K)
///     g (float): Gravitational acceleration (m/s²)
///     h_e (float): Characteristic height (m)
///
/// Returns:
///     float: Non-dimensional heat release rate (dimensionless)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> # Calculate non-dimensional heat release rate for a 1000 kW fire
///     >>> q_star = ofire.tr_17.section_2.equation_1.calculate_nondime_hrr(
///     ...     q_dot=1000.0,    # kW
///     ...     rho_a=1.2,       # kg/m³
///     ...     c_p=1.0,         # kJ/kg·K
///     ...     t_a=293.0,       # K
///     ...     g=9.8,           # m/s²
///     ...     h_e=3.0          # m
///     ... )
///     >>> print(f"Q* = {q_star:.4f}")
fn calculate_nondime_hrr(
    q_dot: f64,
    rho_a: f64,
    c_p: f64,
    t_a: f64,
    g: f64,
    h_e: f64,
) -> PyResult<f64> {
    Ok(rust_equation_1::calculate_nondime_hrr(
        q_dot, rho_a, c_p, t_a, g, h_e,
    ))
}

pub fn equation_1_intro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_nondime_hrr, m)?)?;
    Ok(())
}
