use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use ::openfire::fire_dynamics_tools::chapter_2::equation_2_1 as rust_equation_2_1;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_2 as rust_equation_2_2;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_3 as rust_equation_2_3;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_4 as rust_equation_2_4;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_5 as rust_equation_2_5;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_6 as rust_equation_2_6;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_7 as rust_equation_2_7;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_8 as rust_equation_2_8;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_9 as rust_equation_2_9;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_10 as rust_equation_2_10;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_11 as rust_equation_2_11;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_12 as rust_equation_2_12;
use ::openfire::fire_dynamics_tools::chapter_2::equation_2_13 as rust_equation_2_13;

#[pyfunction]
/// Calculate hot gas temperature increase for natural ventilation using the MQH method (Equation 2.1).
///
/// This function computes the temperature increase of hot gases in naturally ventilated
/// enclosures based on the MQH (Mass, Quality, Heat) correlation method.
///
/// .. math::
///
///    \Delta T_g = 6.85 \cdot \left( \frac{Q^2}{\left(\sum_i A_v \cdot \sqrt{H_v}\right) \cdot (A_t \cdot h_k)} \right)^{1/3}
///
/// where:
///
/// - :math:`\Delta T_g` is the hot gas temperature increase (K)
/// - :math:`Q` is the heat release rate (kW)
/// - :math:`A_v` is the ventilation opening area (m²)
/// - :math:`H_v` is the ventilation opening height (m)
/// - :math:`A_t` is the total interior surface area (m²)
/// - :math:`h_k` is the heat transfer coefficient (kW/m²K)
///
/// Args:
///     q (list[float]): Heat release rates (kW)
///     a_v (list[float]): Ventilation opening areas (m²)
///     h_v (list[float]): Ventilation opening heights (m)
///     a_t (float): Total interior surface area (m²)
///     h_k (float): Heat transfer coefficient (kW/m²K)
///
/// Returns:
///     list[float]: Hot gas temperature increases (K)
///
/// Example:
///     >>> import ofire
///     >>> q = [500.0, 1000.0, 1500.0]
///     >>> a_v = [2.5, 1.5]
///     >>> h_v = [2.0, 1.0]
///     >>> a_t = 75.0
///     >>> h_k = 0.035
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_1.hot_gas_temperature_increase_natural_ventilation_mqh(q, a_v, h_v, a_t, h_k)
fn hot_gas_temperature_increase_natural_ventilation_mqh(
    q: Vec<f64>,
    a_v: Vec<f64>,
    h_v: Vec<f64>,
    a_t: f64,
    h_k: f64,
) -> PyResult<Vec<f64>> {
    Ok(rust_equation_2_1::hot_gas_temperature_increase_natural_ventilation_mqh(q, a_v, h_v, a_t, h_k))
}

#[pyfunction]
/// Calculate compartment interior surface area (Equation 2.2).
///
/// This function computes the total interior surface area of a compartment
/// by calculating the areas of all walls, floor, and ceiling, then subtracting
/// the ventilation opening area.
///
/// .. math::
///
///    A_t = 2 \cdot (W_c \cdot L_c) + 2 \cdot (H_c \cdot W_c) + 2 \cdot (H_c \cdot L_c) - A_v
///
/// where:
///
/// - :math:`A_t` is the total interior surface area (m²)
/// - :math:`W_c` is the compartment width (m)
/// - :math:`L_c` is the compartment length (m)
/// - :math:`H_c` is the compartment height (m)
/// - :math:`A_v` is the ventilation opening area (m²)
///
/// Args:
///     w_c (float): Compartment width (m)
///     l_c (float): Compartment length (m)
///     h_c (float): Compartment height (m)
///     a_v (float): Ventilation opening area (m²)
///
/// Returns:
///     float: Total interior surface area (m²)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_2.compartment_interior_surface_area(7.5, 4.0, 2.75, 4.5)
fn compartment_interior_surface_area(
    w_c: f64,
    l_c: f64,
    h_c: f64,
    a_v: f64,
) -> PyResult<f64> {
    Ok(rust_equation_2_2::comparment_interior_surface_area(w_c, l_c, h_c, a_v))
}

#[pyfunction]
/// Calculate heat transfer coefficient for short times or thick walls (Equation 2.5).
///
/// This function computes the heat transfer coefficient for materials during
/// short exposure times or for thick-walled constructions where the material
/// can be considered as a semi-infinite solid.
///
/// .. math::
///
///    h_k = \left( \frac{k \cdot \rho \cdot c}{t} \right)^{1/2}
///
/// where:
///
/// - :math:`h_k` is the heat transfer coefficient (kW/m²K)
/// - :math:`k` is the thermal conductivity (kW/mK)
/// - :math:`\rho` is the density (kg/m³)
/// - :math:`c` is the specific heat capacity (kJ/kgK)
/// - :math:`t` is the time (s)
///
/// Args:
///     k (float): Thermal conductivity (kW/mK)
///     rho (float): Density (kg/m³)
///     c (float): Specific heat capacity (kJ/kgK)
///     t (float): Time (s)
///
/// Returns:
///     float: Heat transfer coefficient (kW/m²K)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_5.heat_transfer_coefficient_shorttimes_or_thickwalls(0.002, 2400.0, 1.17, 1800.0)
fn heat_transfer_coefficient_shorttimes_or_thickwalls(
    k: f64,
    rho: f64,
    c: f64,
    t: f64,
) -> PyResult<f64> {
    Ok(rust_equation_2_5::heat_transfer_coefficient_shorttimes_or_thickwalls(k, rho, c, t))
}

#[pyfunction]
/// Calculate height of smoke layer interface using Yamana-Tanaka correlation (Equation 2.10).
///
/// This function computes the height of the smoke layer interface in naturally
/// ventilated compartments using the Yamana-Tanaka correlation for transient conditions.
///
/// .. math::
///
///    Z = \left( \frac{2 \cdot k \cdot Q^{1/3} \cdot t}{3 \cdot A_c} + \frac{1}{H_c^{2/3}} \right)^{-3/2}
///
/// where:
///
/// - :math:`Z` is the height of smoke layer interface (m)
/// - :math:`k` is the entrainment coefficient (dimensionless)
/// - :math:`Q` is the heat release rate (kW)
/// - :math:`t` is the time (s)
/// - :math:`A_c` is the compartment floor area (m²)
/// - :math:`H_c` is the compartment height (m)
///
/// Args:
///     k (list[float]): Entrainment coefficients (dimensionless)
///     q (list[float]): Heat release rates (kW)
///     t (list[float]): Time values (s)
///     a_c (float): Compartment floor area (m²)
///     h_c (float): Compartment height (m)
///
/// Returns:
///     list[float]: Heights of smoke layer interface (m)
///
/// Example:
///     >>> import ofire
///     >>> k = [0.15, 0.12, 0.1]
///     >>> q = [500.0, 1000.0, 1500.0]
///     >>> t = [60.0, 90.0, 120.0]
///     >>> a_c = 250.0
///     >>> h_c = 4.5
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_10.height_smoke_layer_interface_natural_ventilation_yamana_tanaka(k, q, t, a_c, h_c)
fn height_smoke_layer_interface_natural_ventilation_yamana_tanaka(
    k: Vec<f64>,
    q: Vec<f64>,
    t: Vec<f64>,
    a_c: f64,
    h_c: f64,
) -> PyResult<Vec<f64>> {
    Ok(rust_equation_2_10::height_smoke_layer_interface_natural_ventilation_yamana_tanaka(k, q, t, a_c, h_c))
}

#[pyfunction]
/// Calculate heat transfer coefficient for long times or thin walls (Equation 2.3).
///
/// This function computes the heat transfer coefficient for materials during
/// long exposure times or for thin-walled constructions where steady-state
/// heat transfer conditions are reached.
///
/// .. math::
///
///    h_k = \frac{k}{\delta}
///
/// where:
///
/// - :math:`h_k` is the heat transfer coefficient (kW/m²K)
/// - :math:`k` is the thermal conductivity (kW/mK)
/// - :math:`\delta` is the material thickness (m)
///
/// Args:
///     k (float): Thermal conductivity (kW/mK)
///     delta (float): Material thickness (m)
///
/// Returns:
///     float: Heat transfer coefficient (kW/m²K)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_3.heat_transfer_coefficient_longtimes_or_thinwalls(0.002, 0.25)
fn heat_transfer_coefficient_longtimes_or_thinwalls(
    k: f64,
    delta: f64,
) -> PyResult<f64> {
    Ok(rust_equation_2_3::heat_transfer_coefficient_longtimes_or_thinwalls(k, delta))
}

#[pyfunction]
/// Calculate thermal penetration time (Equation 2.4).
///
/// This function computes the thermal penetration time, which is the time
/// required for heat to significantly penetrate through a material thickness.
///
/// .. math::
///
///    t_p = \frac{\rho \cdot c_p}{k} \cdot \left( \frac{\delta}{2} \right)^2
///
/// where:
///
/// - :math:`t_p` is the thermal penetration time (s)
/// - :math:`\rho` is the density (kg/m³)
/// - :math:`c_p` is the specific heat capacity (kJ/kgK)
/// - :math:`k` is the thermal conductivity (kW/mK)
/// - :math:`\delta` is the material thickness (m)
///
/// Args:
///     rho (float): Density (kg/m³)
///     c_p (float): Specific heat capacity (kJ/kgK)
///     k (float): Thermal conductivity (kW/mK)
///     delta (float): Material thickness (m)
///
/// Returns:
///     float: Thermal penetration time (s)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_4.thermal_penetration_time(2400.0, 1.17, 0.002, 0.25)
fn thermal_penetration_time(
    rho: f64,
    c_p: f64,
    k: f64,
    delta: f64,
) -> PyResult<f64> {
    Ok(rust_equation_2_4::thermal_penetration_time(rho, c_p, k, delta))
}

#[pyfunction]
/// Calculate hot gas temperature increase for closed compartment using Beyler correlation (Equation 2.6).
///
/// This function computes the temperature increase of hot gases in closed
/// compartments using the Beyler correlation for transient heating conditions.
///
/// .. math::
///
///    \Delta T_g = \frac{2 \cdot \frac{Q}{m \cdot c_p}}{\left(\frac{2 \cdot 0.4 \cdot \sqrt{k \cdot \rho \cdot c}}{m \cdot c_p}\right)^2} \cdot \left(\frac{2 \cdot 0.4 \cdot \sqrt{k \cdot \rho \cdot c}}{m \cdot c_p} \cdot \sqrt{t} - 1 + e^{-\frac{2 \cdot 0.4 \cdot \sqrt{k \cdot \rho \cdot c}}{m \cdot c_p} \cdot \sqrt{t}}\right)
///
/// where:
///
/// - :math:`\Delta T_g` is the hot gas temperature increase (K)
/// - :math:`Q` is the heat release rate (kW)
/// - :math:`m` is the mass flow rate (kg/s)
/// - :math:`c_p` is the specific heat capacity (kJ/kgK)
/// - :math:`k` is the thermal conductivity (kW/mK)
/// - :math:`\rho` is the density (kg/m³)
/// - :math:`c` is the specific heat capacity (kJ/kgK)
/// - :math:`t` is the time (s)
///
/// Args:
///     k (float): Thermal conductivity (kW/mK)
///     rho (float): Density (kg/m³)
///     c (float): Specific heat capacity (kJ/kgK)
///     t (float): Time (s)
///     m (float): Mass flow rate (kg/s)
///     c_p (float): Specific heat capacity (kJ/kgK)
///     q (list[float]): Heat release rates (kW)
///
/// Returns:
///     list[float]: Hot gas temperature increases (K)
///
/// Example:
///     >>> import ofire
///     >>> q = [250.0, 500.0, 750.0]
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_6.hot_gas_temperature_increase_natural_ventilation_beyler_closed_compartment(0.002, 2400.0, 1.17, 60.0, 100.0, 1.0, q)
fn hot_gas_temperature_increase_natural_ventilation_beyler_closed_compartment(
    k: f64,
    rho: f64,
    c: f64,
    t: f64,
    m: f64,
    c_p: f64,
    q: Vec<f64>,
) -> PyResult<Vec<f64>> {
    Ok(rust_equation_2_6::hot_gas_temperature_increase_natural_ventilation_beyler_closed_compartment(k, rho, c, t, m, c_p, q))
}

#[pyfunction]
/// Calculate nondimensional hot gas temperature increase for forced ventilation using FPA correlation (Equation 2.7).
///
/// This function computes the nondimensional temperature increase of hot gases
/// in forced ventilation systems using the Factory Mutual Research Corporation (FPA) correlation.
///
/// .. math::
///
///    \frac{\Delta T_g}{T_a} = 0.63 \cdot \left( \frac{Q}{m \cdot c_p \cdot T_a} \right)^{0.72} \cdot \left( \frac{h_k \cdot A_t}{m \cdot c_p} \right)^{-0.36}
///
/// where:
///
/// - :math:`\Delta T_g / T_a` is the nondimensional hot gas temperature increase (dimensionless)
/// - :math:`Q` is the heat release rate (kW)
/// - :math:`m` is the mass flow rate (kg/s)
/// - :math:`c_p` is the specific heat capacity (kJ/kgK)
/// - :math:`T_a` is the ambient temperature (K)
/// - :math:`h_k` is the heat transfer coefficient (kW/m²K)
/// - :math:`A_t` is the total interior surface area (m²)
///
/// Args:
///     q (list[float]): Heat release rates (kW)
///     m (float): Mass flow rate (kg/s)
///     t_a (float): Ambient temperature (K)
///     h_k (float): Heat transfer coefficient (kW/m²K)
///     a_t (float): Total interior surface area (m²)
///     c_p (float): Specific heat capacity (kJ/kgK)
///
/// Returns:
///     list[float]: Nondimensional hot gas temperature increases (dimensionless)
///
/// Example:
///     >>> import ofire
///     >>> q = [150.0, 300.0, 450.0]
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_7.nondimensional_hot_gas_temperature_increase_forced_ventilation_fpa(q, 2.5, 293.0, 0.035, 100.0, 1.0)
fn nondimensional_hot_gas_temperature_increase_forced_ventilation_fpa(
    q: Vec<f64>,
    m: f64,
    t_a: f64,
    h_k: f64,
    a_t: f64,
    c_p: f64,
) -> PyResult<Vec<f64>> {
    Ok(rust_equation_2_7::nondimensional_hot_gas_temperature_increase_forced_ventilation_fpa(q, m, t_a, h_k, a_t, c_p))
}

#[pyfunction]
/// Calculate hot gas temperature increase for forced ventilation using Deal and Beyler correlation (Equation 2.8).
///
/// This function computes the temperature increase of hot gases in forced
/// ventilation systems using the Deal and Beyler steady-state correlation.
///
/// .. math::
///
///    \Delta T_g = \frac{Q}{m \cdot c_p + h_k \cdot A_t}
///
/// where:
///
/// - :math:`\Delta T_g` is the hot gas temperature increase (K)
/// - :math:`Q` is the heat release rate (kW)
/// - :math:`m` is the mass flow rate (kg/s)
/// - :math:`c_p` is the specific heat capacity (kJ/kgK)
/// - :math:`h_k` is the heat transfer coefficient (kW/m²K)
/// - :math:`A_t` is the total interior surface area (m²)
///
/// Args:
///     q (list[float]): Heat release rates (kW)
///     m (float): Mass flow rate (kg/s)
///     c_p (float): Specific heat capacity (kJ/kgK)
///     h_k (float): Heat transfer coefficient (kW/m²K)
///     a_t (float): Total interior surface area (m²)
///
/// Returns:
///     list[float]: Hot gas temperature increases (K)
///
/// Example:
///     >>> import ofire
///     >>> q = [150.0, 300.0, 450.0]
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_8.hot_gas_temperature_increase_forced_ventilation_deal_and_beyler(q, 2.5, 1.0, 0.035, 100.0)
fn hot_gas_temperature_increase_forced_ventilation_deal_and_beyler(
    q: Vec<f64>,
    m: f64,
    c_p: f64,
    h_k: f64,
    a_t: f64,
) -> PyResult<Vec<f64>> {
    Ok(rust_equation_2_8::hot_gas_temperature_increase_forced_ventilation_deal_and_beyler(q, m, c_p, h_k, a_t))
}

#[pyfunction]
/// Calculate convective heat transfer coefficient (Equation 2.9).
///
/// This function computes the convective heat transfer coefficient as the
/// maximum of the coefficients for short/thick and long/thin wall conditions.
///
/// .. math::
///
///    h_k = 0.4 \cdot \max \left( \left( \frac{k \cdot \rho \cdot c}{t} \right)^{1/2}, \frac{k}{\delta} \right)
///
/// where:
///
/// - :math:`h_k` is the convective heat transfer coefficient (kW/m²K)
/// - :math:`k` is the thermal conductivity (kW/mK)
/// - :math:`\rho` is the density (kg/m³)
/// - :math:`c` is the specific heat capacity (kJ/kgK)
/// - :math:`t` is the time (s)
/// - :math:`\delta` is the material thickness (m)
///
/// Args:
///     k (float): Thermal conductivity (kW/mK)
///     rho (float): Density (kg/m³)
///     c (float): Specific heat capacity (kJ/kgK)
///     t (float): Time (s)
///     delta (float): Material thickness (m)
///
/// Returns:
///     float: Convective heat transfer coefficient (kW/m²K)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_9.convective_heat_transfer_coefficient(0.002, 2400.0, 1.17, 180.0, 0.2)
fn convective_heat_transfer_coefficient(
    k: f64,
    rho: f64,
    c: f64,
    t: f64,
    delta: f64,
) -> PyResult<f64> {
    Ok(rust_equation_2_9::convective_heat_transfer_coefficient(k, rho, c, t, delta))
}

#[pyfunction]
/// Calculate k constant for smoke layer height using Yamana-Tanaka correlation (Equation 2.11).
///
/// This function computes the entrainment coefficient k for the Yamana-Tanaka
/// smoke layer height correlation based on hot gas density and ambient conditions.
///
/// .. math::
///
///    k = \frac{0.21}{\rho_g} \cdot \left( \frac{\rho_a^2 \cdot g}{c_p \cdot T_a} \right)^{1/3}
///
/// where:
///
/// - :math:`k` is the entrainment coefficient (dimensionless)
/// - :math:`\rho_g` is the hot gas density (kg/m³)
/// - :math:`\rho_a` is the ambient air density (kg/m³)
/// - :math:`g` is the gravitational acceleration (m/s²)
/// - :math:`c_p` is the specific heat capacity (kJ/kgK)
/// - :math:`T_a` is the ambient temperature (K)
///
/// Args:
///     rho_g (list[float]): Hot gas densities (kg/m³)
///     rho_a (float): Ambient air density (kg/m³)
///     g (float): Gravitational acceleration (m/s²)
///     c_p (float): Specific heat capacity (kJ/kgK)
///     t_a (float): Ambient temperature (K)
///
/// Returns:
///     list[float]: Entrainment coefficients (dimensionless)
///
/// Example:
///     >>> import ofire
///     >>> rho_g = [0.75, 0.5, 0.25]
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_11.k_constant_smoke_layer_height_yamana_tanaka(rho_g, 1.2, 9.81, 1.0, 293.15)
fn k_constant_smoke_layer_height_yamana_tanaka(
    rho_g: Vec<f64>,
    rho_a: f64,
    g: f64,
    c_p: f64,
    t_a: f64,
) -> PyResult<Vec<f64>> {
    Ok(rust_equation_2_11::k_constant_smoke_layer_height_yamana_tanaka(rho_g, rho_a, g, c_p, t_a))
}

#[pyfunction]
/// Calculate k constant for smoke layer height using simplified Yamana-Tanaka correlation (Equation 2.12).
///
/// This function computes the entrainment coefficient k using a simplified
/// version of the Yamana-Tanaka correlation with pre-substituted standard values.
///
/// .. math::
///
///    k = \frac{0.076}{\rho_g}
///
/// where:
///
/// - :math:`k` is the entrainment coefficient (dimensionless)
/// - :math:`\rho_g` is the hot gas density (kg/m³)
/// - 0.076 is derived from substituting standard atmospheric values
///
/// Args:
///     rho_g (list[float]): Hot gas densities (kg/m³)
///
/// Returns:
///     list[float]: Entrainment coefficients (dimensionless)
///
/// Example:
///     >>> import ofire
///     >>> rho_g = [0.75, 0.5, 0.25]
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_12.k_constant_smoke_layer_height_yamana_tanaka_post_substitution(rho_g)
fn k_constant_smoke_layer_height_yamana_tanaka_post_substitution(
    rho_g: Vec<f64>,
) -> PyResult<Vec<f64>> {
    Ok(rust_equation_2_12::k_constant_smoke_layer_height_yamana_tanaka_post_substitution(rho_g))
}

#[pyfunction]
/// Calculate density of hot gas layer (Equation 2.13).
///
/// This function computes the density of the hot gas layer based on the
/// ideal gas law, assuming atmospheric pressure and using the hot gas temperature.
///
/// .. math::
///
///    \rho_g = \frac{353.0}{T_g}
///
/// where:
///
/// - :math:`\rho_g` is the hot gas density (kg/m³)
/// - :math:`T_g` is the hot gas temperature (K)
/// - 353.0 is derived from :math:`\frac{P \cdot M}{R}` at atmospheric conditions
///
/// Args:
///     t_g (list[float]): Hot gas temperatures (K)
///
/// Returns:
///     list[float]: Hot gas densities (kg/m³)
///
/// Example:
///     >>> import ofire
///     >>> t_g = [400.0, 500.0, 600.0]
///     >>> result = ofire.fire_dynamics_tools.chapter_2.equation_2_13.density_hot_gas_layer(t_g)
fn density_hot_gas_layer(t_g: Vec<f64>) -> PyResult<Vec<f64>> {
    Ok(rust_equation_2_13::density_hot_gas_layer(t_g))
}

#[pymodule]
/// Hot gas temperature increase calculations (Equation 2.1).
/// 
/// Natural ventilation calculations using the MQH correlation method.
fn equation_2_1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hot_gas_temperature_increase_natural_ventilation_mqh, m)?)?;
    Ok(())
}

#[pymodule]
/// Compartment interior surface area calculations (Equation 2.2).
/// 
/// Calculate total interior surface area for compartments.
fn equation_2_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compartment_interior_surface_area, m)?)?;
    Ok(())
}

#[pymodule]
/// Heat transfer coefficient calculations for long times (Equation 2.3).
/// 
/// Calculate heat transfer coefficients for long times or thin walls.
fn equation_2_3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(heat_transfer_coefficient_longtimes_or_thinwalls, m)?)?;
    Ok(())
}

#[pymodule]
/// Thermal penetration time calculations (Equation 2.4).
/// 
/// Calculate thermal penetration time for materials.
fn equation_2_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(thermal_penetration_time, m)?)?;
    Ok(())
}

#[pymodule]
/// Heat transfer coefficient calculations for short times (Equation 2.5).
/// 
/// Calculate heat transfer coefficients for short times or thick walls.
fn equation_2_5(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(heat_transfer_coefficient_shorttimes_or_thickwalls, m)?)?;
    Ok(())
}

#[pymodule]
/// Closed compartment temperature calculations (Equation 2.6).
/// 
/// Hot gas temperature increase for closed compartments using Beyler correlation.
fn equation_2_6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hot_gas_temperature_increase_natural_ventilation_beyler_closed_compartment, m)?)?;
    Ok(())
}

#[pymodule]
/// Forced ventilation FPA correlation calculations (Equation 2.7).
/// 
/// Nondimensional hot gas temperature increase for forced ventilation.
fn equation_2_7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(nondimensional_hot_gas_temperature_increase_forced_ventilation_fpa, m)?)?;
    Ok(())
}

#[pymodule]
/// Forced ventilation Deal-Beyler calculations (Equation 2.8).
/// 
/// Hot gas temperature increase for forced ventilation using Deal and Beyler correlation.
fn equation_2_8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hot_gas_temperature_increase_forced_ventilation_deal_and_beyler, m)?)?;
    Ok(())
}

#[pymodule]
/// Convective heat transfer coefficient calculations (Equation 2.9).
/// 
/// Calculate convective heat transfer coefficients.
fn equation_2_9(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convective_heat_transfer_coefficient, m)?)?;
    Ok(())
}

#[pymodule]
/// Smoke layer interface height calculations (Equation 2.10).
/// 
/// Natural ventilation calculations using the Yamana-Tanaka correlation.
fn equation_2_10(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(height_smoke_layer_interface_natural_ventilation_yamana_tanaka, m)?)?;
    Ok(())
}

#[pymodule]
/// Yamana-Tanaka k constant calculations (Equation 2.11).
/// 
/// Calculate entrainment coefficient for smoke layer height.
fn equation_2_11(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(k_constant_smoke_layer_height_yamana_tanaka, m)?)?;
    Ok(())
}

#[pymodule]
/// Simplified Yamana-Tanaka k constant calculations (Equation 2.12).
/// 
/// Calculate entrainment coefficient using simplified correlation.
fn equation_2_12(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(k_constant_smoke_layer_height_yamana_tanaka_post_substitution, m)?)?;
    Ok(())
}

#[pymodule]
/// Hot gas density calculations (Equation 2.13).
/// 
/// Calculate density of hot gas layer based on temperature.
fn equation_2_13(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(density_hot_gas_layer, m)?)?;
    Ok(())
}

#[pymodule]
/// Chapter 2 - General fire dynamics tools and calculations.
/// 
/// This module contains fundamental fire dynamics calculations including
/// hot gas temperatures, smoke layer heights, heat transfer coefficients,
/// and compartment geometry calculations.
/// 
/// These equations are essential for fire safety engineering analysis
/// and compartment fire modeling.
pub fn chapter_2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_2_1))?;
    m.add_wrapped(wrap_pymodule!(equation_2_2))?;
    m.add_wrapped(wrap_pymodule!(equation_2_3))?;
    m.add_wrapped(wrap_pymodule!(equation_2_4))?;
    m.add_wrapped(wrap_pymodule!(equation_2_5))?;
    m.add_wrapped(wrap_pymodule!(equation_2_6))?;
    m.add_wrapped(wrap_pymodule!(equation_2_7))?;
    m.add_wrapped(wrap_pymodule!(equation_2_8))?;
    m.add_wrapped(wrap_pymodule!(equation_2_9))?;
    m.add_wrapped(wrap_pymodule!(equation_2_10))?;
    m.add_wrapped(wrap_pymodule!(equation_2_11))?;
    m.add_wrapped(wrap_pymodule!(equation_2_12))?;
    m.add_wrapped(wrap_pymodule!(equation_2_13))?;
    Ok(())
}
