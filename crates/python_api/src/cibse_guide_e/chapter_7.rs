use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import CIBSE Guide E chapter 7 functions
use openfire::cibse_guide_e::chapter_7::{
    equation_7_2 as rust_equation_7_2, equation_7_3 as rust_equation_7_3,
    equation_7_6 as rust_equation_7_6, equation_7_7 as rust_equation_7_7,
    equation_7_8 as rust_equation_7_8, equation_7_9 as rust_equation_7_9,
};

// Equation 7_2 module functions
#[pyfunction]
/// Calculates the stair capacity.
///
/// This equation determines the maximum number of people that can evacuate
/// through a stairway based on its width and number of floors served.
///
/// .. math::
///
///    P = 200 \cdot W + 50 \cdot (W - 0.3) \cdot (N - 1)
///
/// where:
///
/// - :math:`P` is the stair capacity (persons)
/// - :math:`W` is the width of stair (m)
/// - :math:`N` is the number of floors served (dimensionless)
///
/// Args:
///     w (float): Width of stair (m)
///     n (int): Number of floors served
///
/// Returns:
///     int: Stair capacity (persons)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_7.equation_7_2.stair_capacity(1.2, 6)
///     >>> print(f"{result} persons")
///     465 persons
fn stair_capacity(w: f64, n: i32) -> PyResult<i32> {
    Ok(rust_equation_7_2::stair_capacity(w, n))
}

#[pymodule]
/// Calculates the maximum number of people that can evacuate
/// through a stairway based on its width and number of floors served.
fn equation_7_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stair_capacity, m)?)?;
    Ok(())
}

// Equation 7_3 module functions
#[pyfunction]
/// Calculates the required width of stair.
///
/// This equation determines the minimum stair width required to accommodate
/// a given number of people across multiple floors during evacuation.
///
/// .. math::
///
///    W = \frac{P + 15 \cdot N - 15}{150 + 50 \cdot N}
///
/// where:
///
/// - :math:`W` is the required width of stair (m)
/// - :math:`P` is the number of people (persons)
/// - :math:`N` is the number of floors (dimensionless)
///
/// Args:
///     p (int): Number of people (persons)
///     n (int): Number of floors
///
/// Returns:
///     float: Required width of stair (m)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_7.equation_7_3.required_width_stair(550, 6)
///     >>> print(f"{result:.2f} m")
///     1.39 m
fn required_width_stair(p: i32, n: i32) -> PyResult<f64> {
    Ok(rust_equation_7_3::required_width_stair(p, n))
}

#[pymodule]
/// Calculates the minimum stair width required to accommodate
/// a given number of people during evacuation.
fn equation_7_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(required_width_stair, m)?)?;
    Ok(())
}

// Equation 7_6 module functions
#[pyfunction]
/// Calculates the maximum flow rate of persons.
///
/// This equation determines the maximum flow rate of people that can
/// pass through a given width during evacuation.
///
/// .. math::
///
///    F = 1.333 \cdot W
///
/// where:
///
/// - :math:`F` is the maximum flow rate (persons/s)
/// - :math:`W` is the width (m)
///
/// Args:
///     w (float): Width (m)
///
/// Returns:
///     float: Maximum flow rate (persons/s)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_7.equation_7_6.maximum_flowrate_persons(1.2)
fn maximum_flowrate_persons(w: f64) -> PyResult<f64> {
    Ok(rust_equation_7_6::maximum_flowrate_persons(w))
}

#[pymodule]
/// Calculates the maximum flow rate of people through a given width.
fn equation_7_6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(maximum_flowrate_persons, m)?)?;
    Ok(())
}

// Equation 7_7 module functions
#[pyfunction]
/// Calculates the maximum people in stair.
///
/// This equation determines the maximum number of people that can be
/// accommodated in a stairway at any given time.
///
/// .. math::
///
///    N_c = P \cdot A \cdot S
///
/// where:
///
/// - :math:`N_c` is the maximum people in stair (persons)
/// - :math:`P` is the flow rate (persons/s/m)
/// - :math:`A` is the area per person (m²/person)
/// - :math:`S` is the number of storeys (dimensionless)
///
/// Args:
///     p (float): Flow rate (persons/s/m)
///     a (float): Area per person (m²/person)
///     s (int): Number of storeys
///
/// Returns:
///     int: Maximum people in stair (persons)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_7.equation_7_7.maximum_people_in_stair(3.5, 8.0, 6)
fn maximum_people_in_stair(p: f64, a: f64, s: i32) -> PyResult<i32> {
    Ok(rust_equation_7_7::maximum_people_in_stair(p, a, s))
}

#[pymodule]
/// Calculates the maximum number of people that can be accommodated in a stairway.
fn equation_7_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(maximum_people_in_stair, m)?)?;
    Ok(())
}

// Equation 7_8 module functions
#[pyfunction]
/// Calculates the exit capacity of stair.
///
/// This equation determines the number of people that can exit through
/// a stairway within a given time period.
///
/// .. math::
///
///    N_{in} = 1.333 \cdot W_s \cdot T + 3.5 \cdot A \cdot (S - 1)
///
/// where:
///
/// - :math:`N_{in}` is the exit capacity (persons)
/// - :math:`W_s` is the width of stair (m)
/// - :math:`T` is the time period (s)
/// - :math:`A` is the area (m²)
/// - :math:`S` is the number of storeys (dimensionless)
///
/// Args:
///     w_s (float): Width of stair (m)
///     t (float): Time period (s)
///     a (float): Area (m²)
///     s (int): Number of storeys
///
/// Returns:
///     int: Exit capacity (persons)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_7.equation_7_8.exit_capacity_stair(1.2, 150.0, 10.0, 5)
fn exit_capacity_stair(w_s: f64, t: f64, a: f64, s: i32) -> PyResult<i32> {
    Ok(rust_equation_7_8::exit_capacity_stair(w_s, t, a, s))
}

#[pymodule]
/// Calculates the number of people that can exit through a stairway within a given time.
fn equation_7_8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(exit_capacity_stair, m)?)?;
    Ok(())
}

// Equation 7_9 module functions
#[pyfunction]
/// Calculates the acceptance capacity of stair.
///
/// This equation determines the number of people that can be accepted
/// into a stairway from multiple floors during evacuation.
///
/// .. math::
///
///    N_{in} = 1.2 \cdot W_e \cdot T + \rho \cdot A \cdot (S - 1)
///
/// where:
///
/// - :math:`N_{in}` is the acceptance capacity (persons)
/// - :math:`W_e` is the width of entrance (m)
/// - :math:`T` is the time period (s)
/// - :math:`\rho` is the density (persons/m²)
/// - :math:`A` is the area (m²)
/// - :math:`S` is the number of storeys (dimensionless)
///
/// Args:
///     w_e (float): Width of entrance (m)
///     t (float): Time period (s)
///     rho (float): Density (persons/m²)
///     a (float): Area (m²)
///     s (int): Number of storeys
///
/// Returns:
///     int: Acceptance capacity (persons)
///
/// Assumptions:
///     To be completed
///
/// Limitations:
///     To be completed
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.cibse_guide_e.chapter_7.equation_7_9.acceptance_capacity_stair(0.9, 150.0, 2.0, 10.0, 5)
fn acceptance_capacity_stair(w_e: f64, t: f64, rho: f64, a: f64, s: i32) -> PyResult<i32> {
    Ok(rust_equation_7_9::acceptance_capacity_stair(
        w_e, t, rho, a, s,
    ))
}

#[pymodule]
/// Calculates the number of people that can be accepted into a stairway from multiple floors.
fn equation_7_9(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(acceptance_capacity_stair, m)?)?;
    Ok(())
}

#[pymodule]
/// This chapter contains equations for calculating stair capacities,
/// flow rates, and evacuation parameters for egress design.
pub fn chapter_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_7_2))?;
    m.add_wrapped(wrap_pymodule!(equation_7_3))?;
    m.add_wrapped(wrap_pymodule!(equation_7_6))?;
    m.add_wrapped(wrap_pymodule!(equation_7_7))?;
    m.add_wrapped(wrap_pymodule!(equation_7_8))?;
    m.add_wrapped(wrap_pymodule!(equation_7_9))?;
    Ok(())
}
