use wasm_bindgen::prelude::*;

// Import the Rust implementations
use ::fire_dynamics_tools::chapter_2::equation_2_1 as rust_equation_2_1;
use ::fire_dynamics_tools::chapter_2::equation_2_2 as rust_equation_2_2;
use ::fire_dynamics_tools::chapter_2::equation_2_3 as rust_equation_2_3;
use ::fire_dynamics_tools::chapter_2::equation_2_4 as rust_equation_2_4;
use ::fire_dynamics_tools::chapter_2::equation_2_5 as rust_equation_2_5;
use ::fire_dynamics_tools::chapter_2::equation_2_6 as rust_equation_2_6;
use ::fire_dynamics_tools::chapter_2::equation_2_7 as rust_equation_2_7;
use ::fire_dynamics_tools::chapter_2::equation_2_8 as rust_equation_2_8;
use ::fire_dynamics_tools::chapter_2::equation_2_9 as rust_equation_2_9;
use ::fire_dynamics_tools::chapter_2::equation_2_10 as rust_equation_2_10;
use ::fire_dynamics_tools::chapter_2::equation_2_11 as rust_equation_2_11;
use ::fire_dynamics_tools::chapter_2::equation_2_12 as rust_equation_2_12;
use ::fire_dynamics_tools::chapter_2::equation_2_13 as rust_equation_2_13;

/// Calculate hot gas temperature increase for natural ventilation using the MQH method (Equation 2.1).
/// 
/// This function computes the temperature increase of hot gases in naturally ventilated
/// enclosures based on the MQH (Mass, Quality, Heat) correlation method.
/// 
/// @param q - Heat release rate (kW)
/// @param a_v - Ventilation opening areas (m²) as a JavaScript array
/// @param h_v - Ventilation opening heights (m) as a JavaScript array  
/// @param a_t - Total interior surface area (m²)
/// @param h_k - Heat transfer coefficient (kW/m²K)
/// @returns Hot gas temperature increase (K)
#[wasm_bindgen]
pub fn hot_gas_temperature_increase(
    q: f64,
    a_v: &[f64],
    h_v: &[f64],
    a_t: f64,
    h_k: f64,
) -> f64 {
    rust_equation_2_1::hot_gas_temperature_increase(q, a_v.to_vec(), h_v.to_vec(), a_t, h_k)
}

/// Calculate compartment interior surface area (Equation 2.2).
/// 
/// This function computes the total interior surface area of a compartment
/// by calculating the areas of all walls, floor, and ceiling, then subtracting
/// the ventilation opening area.
/// 
/// @param w_c - Compartment width (m)
/// @param l_c - Compartment length (m)
/// @param h_c - Compartment height (m)
/// @param a_v - Ventilation opening area (m²)
/// @returns Total interior surface area (m²)
#[wasm_bindgen]
pub fn compartment_interior_surface_area(w_c: f64, l_c: f64, h_c: f64, a_v: f64) -> f64 {
    rust_equation_2_2::comparment_interior_surface_area(w_c, l_c, h_c, a_v)
}

/// Calculate heat transfer coefficient for short times or thick walls (Equation 2.5).
/// 
/// This function computes the heat transfer coefficient for materials during
/// short exposure times or for thick-walled constructions where the material
/// can be considered as a semi-infinite solid.
/// 
/// @param k - Thermal conductivity (kW/mK)
/// @param rho - Density (kg/m³)
/// @param c - Specific heat capacity (kJ/kgK)
/// @param t - Time (s)
/// @returns Heat transfer coefficient (kW/m²K)
#[wasm_bindgen]
pub fn heat_transfer_coefficient_shorttimes_or_thickwalls(
    k: f64,
    rho: f64,
    c: f64,
    t: f64,
) -> f64 {
    rust_equation_2_5::heat_transfer_coefficient_shorttimes_or_thickwalls(k, rho, c, t)
}

/// Calculate height of smoke layer interface using Yamana-Tanaka correlation (Equation 2.10).
/// 
/// This function computes the height of the smoke layer interface in naturally
/// ventilated compartments using the Yamana-Tanaka correlation for transient conditions.
/// 
/// @param k - Entrainment coefficient (dimensionless)
/// @param q - Heat release rate (kW)
/// @param t - Time (s)
/// @param a_c - Compartment floor area (m²)
/// @param h_c - Compartment height (m)
/// @returns Height of smoke layer interface (m)
#[wasm_bindgen]
pub fn height_smoke_layer_interface_natural_ventilation(
    k: f64,
    q: f64,
    t: f64,
    a_c: f64,
    h_c: f64,
) -> f64 {
    rust_equation_2_10::height_smoke_layer_interface_natural_ventilation(k, q, t, a_c, h_c)
}

/// Calculate heat transfer coefficient for long times or thin walls (Equation 2.3).
/// 
/// This function computes the heat transfer coefficient for materials during
/// long exposure times or for thin-walled constructions where steady-state
/// heat transfer conditions are reached.
/// 
/// @param k - Thermal conductivity (kW/mK)
/// @param delta - Material thickness (m)
/// @returns Heat transfer coefficient (kW/m²K)
#[wasm_bindgen]
pub fn heat_transfer_coefficient_longtimes_or_thinwalls(k: f64, delta: f64) -> f64 {
    rust_equation_2_3::heat_transfer_coefficient_longtimes_or_thinwalls(k, delta)
}

/// Calculate thermal penetration time (Equation 2.4).
/// 
/// This function computes the thermal penetration time, which is the time
/// required for heat to significantly penetrate through a material thickness.
/// 
/// @param rho - Density (kg/m³)
/// @param c_p - Specific heat capacity (kJ/kgK)
/// @param k - Thermal conductivity (kW/mK)
/// @param delta - Material thickness (m)
/// @returns Thermal penetration time (s)
#[wasm_bindgen]
pub fn thermal_penetration_time(rho: f64, c_p: f64, k: f64, delta: f64) -> f64 {
    rust_equation_2_4::thermal_penetration_time(rho, c_p, k, delta)
}

/// Calculate hot gas temperature increase using the Beyler correlation for closed compartments (Equation 2.6).
/// 
/// This function calculates the temperature increase of hot gases in a closed
/// compartment using Beyler's correlation, accounting for thermal properties
/// of the enclosure and fire characteristics.
/// 
/// @param k - Thermal conductivity (kW/mK)
/// @param rho - Density (kg/m³)
/// @param c - Specific heat capacity of internal lining (kJ/kgK)
/// @param t - Time (s)
/// @param m - Mass flow rate (kg/s)
/// @param c_p - Specific heat capacity of air (kJ/kgK)
/// @param q - Heat release rate (kW)
/// @returns Hot gas temperature increase (K)
#[wasm_bindgen]
pub fn hot_gas_temperature_increase_beyler_closed_compartment(
    k: f64,
    rho: f64,
    c: f64,
    t: f64,
    m: f64,
    c_p: f64,
    q: f64,
) -> f64 {
    rust_equation_2_6::hot_gas_temperature_increase(k, rho, c, t, m, c_p, q)
}

/// Calculate nondimensional hot gas temperature increase for forced ventilation using FPA correlation (Equation 2.7).
/// 
/// This function computes the nondimensional temperature increase of hot gases
/// in forced ventilation systems using the Factory Mutual Research Corporation (FPA) correlation.
/// 
/// @param q - Heat release rate (kW)
/// @param m - Mass flow rate (kg/s)
/// @param t_a - Ambient temperature (K)
/// @param h_k - Heat transfer coefficient (kW/m²K)
/// @param a_t - Total interior surface area (m²)
/// @param c_p - Specific heat capacity (kJ/kgK)
/// @returns Nondimensional hot gas temperature increase (dimensionless)
#[wasm_bindgen]
pub fn nondimensional_hot_gas_temperature_increase(
    q: f64,
    m: f64,
    t_a: f64,
    h_k: f64,
    a_t: f64,
    c_p: f64,
) -> f64 {
    rust_equation_2_7::nondimensional_hot_gas_temperature_increase(q, m, t_a, h_k, a_t, c_p)
}

/// Calculate hot gas temperature increase for forced ventilation using Deal and Beyler correlation (Equation 2.8).
/// 
/// This function computes the temperature increase of hot gases in forced
/// ventilation systems using the Deal and Beyler steady-state correlation.
/// 
/// @param q - Heat release rate (kW)
/// @param m - Mass flow rate (kg/s)
/// @param c_p - Specific heat capacity (kJ/kgK)
/// @param h_k - Heat transfer coefficient (kW/m²K)
/// @param a_t - Total interior surface area (m²)
/// @returns Hot gas temperature increase (K)
#[wasm_bindgen]
pub fn hot_gas_temperature_increase_forced_ventilation(
    q: f64,
    m: f64,
    c_p: f64,
    h_k: f64,
    a_t: f64,
) -> f64 {
    rust_equation_2_8::hot_gas_temperature_increase(q, m, c_p, h_k, a_t)
}

/// Calculate convective heat transfer coefficient (Equation 2.9).
/// 
/// This function computes the convective heat transfer coefficient as the
/// maximum of the coefficients for short/thick and long/thin wall conditions.
/// 
/// @param k - Thermal conductivity (kW/mK)
/// @param rho - Density (kg/m³)
/// @param c - Specific heat capacity (kJ/kgK)
/// @param t - Time (s)
/// @param delta - Material thickness (m)
/// @returns Convective heat transfer coefficient (kW/m²K)
#[wasm_bindgen]
pub fn convective_heat_transfer_coefficient(
    k: f64,
    rho: f64,
    c: f64,
    t: f64,
    delta: f64,
) -> f64 {
    rust_equation_2_9::convective_heat_transfer_coefficient(k, rho, c, t, delta)
}

/// Calculate k constant for smoke layer height using Yamana-Tanaka correlation (Equation 2.11).
/// 
/// This function computes the entrainment coefficient k for the Yamana-Tanaka
/// smoke layer height correlation based on hot gas density and ambient conditions.
/// 
/// @param rho_g - Hot gas density (kg/m³)
/// @param rho_a - Ambient air density (kg/m³)
/// @param g - Gravitational acceleration (m/s²)
/// @param c_p - Specific heat capacity (kJ/kgK)
/// @param t_a - Ambient temperature (K)
/// @returns Entrainment coefficient (dimensionless)
#[wasm_bindgen]
pub fn k_constant_smoke_layer_height(
    rho_g: f64,
    rho_a: f64,
    g: f64,
    c_p: f64,
    t_a: f64,
) -> f64 {
    rust_equation_2_11::k_constant_smoke_layer_height(rho_g, rho_a, g, c_p, t_a)
}

/// Calculate k constant for smoke layer height using simplified Yamana-Tanaka correlation (Equation 2.12).
/// 
/// This function computes the entrainment coefficient k using a simplified
/// version of the Yamana-Tanaka correlation with pre-substituted standard values.
/// 
/// @param rho_g - Hot gas density (kg/m³)
/// @returns Entrainment coefficient (dimensionless)
#[wasm_bindgen]
pub fn k_constant_smoke_layer_height_post_substitution(rho_g: f64) -> f64 {
    rust_equation_2_12::k_constant_smoke_layer_height(rho_g)
}

/// Calculate density of hot gas layer (Equation 2.13).
/// 
/// This function computes the density of the hot gas layer based on the
/// ideal gas law, assuming atmospheric pressure and using the hot gas temperature.
/// 
/// @param t_g - Hot gas temperature (K)
/// @returns Hot gas density (kg/m³)
#[wasm_bindgen]
pub fn density_hot_gas_layer(t_g: f64) -> f64 {
    rust_equation_2_13::density_hot_gas_layer(t_g)
}