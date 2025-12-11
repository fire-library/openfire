mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Fire safety engineering modules - starting with just fire_dynamics_tools for POC
pub mod fire_dynamics_tools;

// BR 187 - External Fire Spread module
pub mod br_187;

/**
 * @namespace Utility_Functions
 * @description 
 * Utility and example functions for basic calculations and demonstrations.
 */

/**
 * Calculate the area of a circle
 * 
 * This is a minimal example function to demonstrate WASM documentation generation.
 * 
 * @memberof Utility_Functions
 * @param {number} radius - The radius of the circle in meters
 * @returns {number} The area of the circle in square meters
 * 
 * @example
 * ```javascript
 * import init, { calculate_circle_area } from './pkg/wasm_api.js';
 * 
 * await init();
 * const area = calculate_circle_area(5.0);
 * console.log(`Circle area: ${area} m²`);
 * ```
 */
#[wasm_bindgen]
pub fn calculate_circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

/**
 * Fire safety calculation example
 * 
 * Calculate fire spread risk based on building separation distance.
 * This demonstrates a more domain-specific function for fire safety engineering.
 * 
 * @memberof Utility_Functions
 * @param {number} separation_distance - Distance between buildings in meters
 * @param {number} building_height - Height of the source building in meters
 * @returns {number} Risk factor (0.0 = no risk, 1.0 = maximum risk)
 */
#[wasm_bindgen]
pub fn calculate_fire_spread_risk(separation_distance: f64, building_height: f64) -> f64 {
    // Simplified calculation for demonstration
    let critical_distance = building_height * 0.5;
    if separation_distance >= critical_distance {
        0.0
    } else {
        1.0 - (separation_distance / critical_distance)
    }
}

/**
 * @fileoverview OpenFire WASM API - Fire Safety Engineering Tools
 * 
 * This module provides WebAssembly bindings for fire safety engineering calculations,
 * organized by source document and chapter for easy navigation.
 */

/**
 * @namespace API_Reference
 * @description 
 * Complete API reference for all OpenFire modules, organized by source document.
 * 
 * Available modules:
 * - Fire Dynamics Tools - General fire dynamics calculations
 * - BR 187 - External Fire Spread
 * - PD 7974 - Fire Safety Engineering Principles (coming soon)
 * - BS 9999 - Fire Safety in Buildings (coming soon)
 * - CIBSE Guide E - Fire Safety Engineering (coming soon)
 */

/**
 * Get the Fire Dynamics Tools namespace
 * 
 * Returns an instance that provides access to organized fire dynamics calculations
 * by document chapter.
 * 
 * @memberof API_Reference
 * @returns {FireDynamicsTools} FireDynamicsTools instance with chapter-organized functions
 * 
 * @example
 * ```javascript
 * import init, { get_fire_dynamics_tools } from './pkg/wasm_api.js';
 * 
 * await init();
 * const fdt = get_fire_dynamics_tools();
 * const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
 * console.log(`Temperature increase: ${tempIncrease} K`);
 * ```
 */
#[wasm_bindgen]
pub fn get_fire_dynamics_tools() -> FireDynamicsTools {
    FireDynamicsTools
}

/**
 * Get the BR 187 namespace
 * 
 * Returns an instance that provides access to organized BR 187 External Fire Spread calculations
 * by document appendix and equation.
 * 
 * @memberof API_Reference
 * @returns {BR187} BR187 instance with appendix-organized functions
 * 
 * @example
 * ```javascript
 * import init, { get_br_187 } from './pkg/wasm_api.js';
 * 
 * await init();
 * const br187 = get_br_187();
 * const intensity = br187.appendix_a.equation_a1.radiation_intensity(5.67e-11, 0.9, 1273.15);
 * console.log(`Radiation intensity: ${intensity} kW/m²`);
 * ```
 */
#[wasm_bindgen]
pub fn get_br_187() -> BR187 {
    BR187
}

/**
 * Fire Dynamics Tools
 * 
 * Contains functions from various chapters of fire dynamics engineering documents.
 * Organized by chapter for easy navigation and reference.
 * 
 * @memberof API_Reference
 * @namespace FireDynamicsTools
 */
#[wasm_bindgen]
pub struct FireDynamicsTools;

/**
 * BR 187 - External Fire Spread
 * 
 * Contains functions from BR 187 External Fire Spread document.
 * Organized by appendix and equation for easy navigation and reference.
 * 
 * @memberof API_Reference
 * @namespace BR187
 */
#[wasm_bindgen]
pub struct BR187;

#[wasm_bindgen]
impl FireDynamicsTools {
    /**
     * Chapter 2 - Compartment Fire Dynamics
     * 
     * @memberof API_Reference.FireDynamicsTools
     * @returns {Chapter2} Chapter2 namespace with compartment fire calculations
     */
    #[wasm_bindgen(getter)]
    pub fn chapter_2() -> Chapter2 {
        Chapter2
    }
    
    /**
     * Chapter 4 - Heat Transfer
     * 
     * @memberof API_Reference.FireDynamicsTools  
     * @returns {Chapter4} Chapter4 namespace with heat transfer calculations
     */
    #[wasm_bindgen(getter)]
    pub fn chapter_4() -> Chapter4 {
        Chapter4
    }
    
    /**
     * Chapter 5 - Smoke Movement
     * 
     * @memberof API_Reference.FireDynamicsTools
     * @returns {Chapter5} Chapter5 namespace with smoke movement calculations
     */
    #[wasm_bindgen(getter)]
    pub fn chapter_5() -> Chapter5 {
        Chapter5
    }
}

/// Chapter 2 - Compartment Fire Dynamics
#[wasm_bindgen]
pub struct Chapter2;

#[wasm_bindgen]
impl Chapter2 {
    /// Calculate hot gas temperature increase (Equation 2.1)
    /// 
    /// Calculates the temperature increase of hot gases in a compartment fire.
    /// Simplified version for demonstration of namespace organization.
    /// 
    /// # Arguments
    /// * `heat_release_rate` - Heat release rate in kW
    /// * `compartment_area` - Floor area of compartment in m²
    /// * `compartment_height` - Height of compartment in m
    /// 
    /// # Returns
    /// Temperature increase in Kelvin
    #[wasm_bindgen]
    pub fn hot_gas_temperature_increase(&self, heat_release_rate: f64, compartment_area: f64, compartment_height: f64) -> f64 {
        // Simplified calculation for demonstration
        // In real implementation, this would call the complex fire_dynamics_tools function
        6.85 * (heat_release_rate.powf(2.0) / (compartment_area * compartment_height)).powf(1.0/3.0)
    }
    
    /// Calculate density of hot gas layer (Equation 2.2)
    /// 
    /// Calculates the density of the hot gas layer based on temperature.
    /// 
    /// # Arguments
    /// * `hot_gas_temperature` - Temperature of hot gas layer in Kelvin
    /// 
    /// # Returns
    /// Density in kg/m³
    #[wasm_bindgen]
    pub fn density_hot_gas_layer(&self, hot_gas_temperature: f64) -> f64 {
        // Simplified ideal gas law calculation for demonstration
        // ρ = P/(R*T), assuming standard pressure
        101325.0 / (287.0 * hot_gas_temperature)
    }
}

/// Chapter 4 - Heat Transfer
#[wasm_bindgen]
pub struct Chapter4;

#[wasm_bindgen]
impl Chapter4 {
    /// Heat transfer coefficient for short times or thick walls (Equation 4.1)
    /// 
    /// # Arguments
    /// * `thermal_conductivity` - Thermal conductivity in W/(m·K)
    /// * `thermal_penetration_time` - Thermal penetration time in seconds
    /// 
    /// # Returns
    /// Heat transfer coefficient in W/(m²·K)
    #[wasm_bindgen]
    pub fn heat_transfer_coefficient_short_times(&self, thermal_conductivity: f64, thermal_penetration_time: f64) -> f64 {
        // Note: Using placeholder since actual equation isn't implemented yet
        thermal_conductivity / thermal_penetration_time.sqrt()
    }
}

/// Chapter 5 - Smoke Movement
#[wasm_bindgen]
pub struct Chapter5;

#[wasm_bindgen]
impl Chapter5 {
    /// Calculate thermal penetration time (Equation 5.1)
    /// 
    /// # Arguments
    /// * `thermal_diffusivity` - Thermal diffusivity in m²/s
    /// * `thickness` - Wall thickness in meters
    /// 
    /// # Returns
    /// Thermal penetration time in seconds
    #[wasm_bindgen]
    pub fn thermal_penetration_time(&self, thermal_diffusivity: f64, thickness: f64) -> f64 {
        // Note: Using placeholder calculation
        (thickness * thickness) / thermal_diffusivity
    }
}

#[wasm_bindgen]
impl BR187 {
    /**
     * Appendix A - Thermal Radiation Calculations
     * 
     * @memberof API_Reference.BR187
     * @returns {AppendixA} AppendixA namespace with thermal radiation calculations
     */
    #[wasm_bindgen(getter)]
    pub fn appendix_a() -> br_187::AppendixA {
        br_187::AppendixA
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}