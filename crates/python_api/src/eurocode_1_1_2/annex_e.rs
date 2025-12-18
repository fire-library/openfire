use pyo3::prelude::*;

// Import Eurocode 1-1-2 annex_e equation_e_1 functions
use openfire::eurocode_1_1_2::annex_e::{
    equation_e_1 as rust_equation_e_1,
};

#[pyfunction]
/// Calculate the design fire load.
///
/// The design fire load is calculated by applying various safety factors 
/// and modification factors to the characteristic fire load:
///
/// .. math::
///
///    q_{f,d} = q_{f,k} \cdot m \cdot \delta_{q1} \cdot \delta_{q2} \cdot \delta_n
///
/// where:
///
/// - :math:`q_{f,d}` is the design fire load (MJ/m²)
/// - :math:`q_{f,k}` is the characteristic fire load (MJ/m²)
/// - :math:`m` is the combustion factor (dimensionless)
/// - :math:`\delta_{q1}` is a factor taking into account the fire activation risk due to the size of the compartment (dimensionless)
/// - :math:`\delta_{q2}` is a factor taking into account the fire activation risk due to the type of occupancy (dimensionless)
/// - :math:`\delta_n` is a factor taking into account the different active fire fighting measures (dimensionless)
///
/// Args:
///     q_f_k (float): Characteristic fire load (MJ/m²)
///     m (float): Combustion factor (dimensionless)
///     delta_q_1 (float): Factor taking into account the fire activation risk due to the size of the compartment (dimensionless)
///     delta_q_2 (float): Factor taking into account the fire activation risk due to the type of occupancy (dimensionless)
///     delta_n (float): Factor taking into account the different active fire fighting measures (dimensionless)
///
/// Returns:
///     float: Design fire load (MJ/m²)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.eurocode_1_1_2.annex_e.design_fire_load(780.0, 0.8, 1.1, 0.78, 0.123)
///     >>> print(f"{result:.2f} MJ/m²")
fn design_fire_load(q_f_k: f64, m: f64, delta_q_1: f64, delta_q_2: f64, delta_n: f64) -> PyResult<f64> {
    Ok(rust_equation_e_1::design_fire_load(q_f_k, m, delta_q_1, delta_q_2, delta_n))
}

#[pyfunction]
/// Calculate the combination factor δₙ for the fire load, which accounts for the different active fire fighting measures.
///
/// The combination factor is the product of all individual factors:
///
/// .. math::
///
///    \delta_n = \delta_{n1} \cdot \delta_{n2} \cdot \delta_{n3} \cdot \delta_{n4} \cdot \delta_{n5} \cdot \delta_{n6q} \cdot \delta_{n7} \cdot \delta_{n8} \cdot \delta_{n9} \cdot \delta_{n10}
///
/// where:
///
/// - :math:`\delta_{n1}` is the factor for automatic water extinguishing systems (dimensionless)
/// - :math:`\delta_{n2}` is the factor for independent water supplies (dimensionless)
/// - :math:`\delta_{n3}` is the factor for automatic fire detection & alarm by heat (dimensionless)
/// - :math:`\delta_{n4}` is the factor for automatic fire detection & alarm by smoke (dimensionless)
/// - :math:`\delta_{n5}` is the factor for automatic alarm transmission to fire brigade (dimensionless)
/// - :math:`\delta_{n6}` is the factor for work fire brigade (dimensionless)
/// - :math:`\delta_{n7}` is the factor for off site fire brigade (dimensionless)
/// - :math:`\delta_{n8}` is the factor for safe access routes times (dimensionless)
/// - :math:`\delta_{n9}` is the factor for fire fighting devices (dimensionless)
/// - :math:`\delta_{n10}` is the factor for smoke exhaust system (dimensionless)
///
/// Args:
///     delta_n_1 (float): Factor for automatic water extinguishing systems (dimensionless)
///     delta_n_2 (float): Factor for independent water supplies (dimensionless)
///     delta_n_3 (float): Factor for automatic fire detection & alarm by heat (dimensionless)
///     delta_n_4 (float): Factor for automatic fire detection & alarm by smoke (dimensionless)
///     delta_n_5 (float): Factor for automatic alarm transmission to fire brigade (dimensionless)
///     delta_n_6q (float): Factor for work fire brigade (dimensionless)
///     delta_n_7 (float): Factor for off site fire brigade (dimensionless)
///     delta_n_8 (float): Factor for safe access routes times (dimensionless)
///     delta_n_9 (float): Factor for fire fighting devices (dimensionless)
///     delta_n_10 (float): Factor for smoke exhaust system (dimensionless)
///
/// Returns:
///     float: Combined factor δₙ (dimensionless)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.eurocode_1_1_2.annex_e.delta_n(0.61, 1.0, 0.87, 0.87, 0.87, 0.61, 0.61, 0.9, 1.0, 1.0)
///     >>> print(f"δₙ = {result:.4f}")
fn delta_n(
    delta_n_1: f64,
    delta_n_2: f64,
    delta_n_3: f64,
    delta_n_4: f64,
    delta_n_5: f64,
    delta_n_6: f64,
    delta_n_7: f64,
    delta_n_8: f64,
    delta_n_9: f64,
    delta_n_10: f64,
) -> PyResult<f64> {
    Ok(rust_equation_e_1::delta_n(
        delta_n_1, delta_n_2, delta_n_3, delta_n_4, delta_n_5,
        delta_n_6, delta_n_7, delta_n_8, delta_n_9, delta_n_10,
    ))
}

#[pymodule]
/// This module contains equations from Annex E of Eurocode 1-1-2 for calculating
/// design fire loads in buildings.
///
/// Available equations:
///     design_fire_load: Calculate the design fire load (Equation E-1)
///     delta_n: Calculate the combination factor for fire load
///
/// Example:
///     >>> import ofire
///     >>> ofire.eurocode_1_1_2.annex_e.design_fire_load
pub fn annex_e(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(design_fire_load, m)?)?;
    m.add_function(wrap_pyfunction!(delta_n, m)?)?;
    Ok(())
}