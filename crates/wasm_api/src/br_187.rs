use wasm_bindgen::prelude::*;

/**
 * @namespace BR187
 * @description 
 * BR 187 - External Fire Spread calculations.
 * 
 * Contains equations and methods for calculating external fire spread 
 * parameters as defined in BR 187 standard.
 */

/**
 * BR 187 Appendix A
 * 
 * Contains radiation and configuration factor calculations for external fire spread.
 * Organized by equation for easy navigation and reference.
 * 
 * @memberof BR187
 * @namespace AppendixA
 */
#[wasm_bindgen]
pub struct AppendixA;

#[wasm_bindgen]
impl AppendixA {
    /**
     * Equation A1 - Stefan-Boltzmann Radiation Intensity
     * 
     * Access to BR 187 Appendix A Equation A1 functions.
     * 
     * @memberof BR187.AppendixA
     * @returns {EquationA1} EquationA1 namespace with radiation intensity calculations
     */
    #[wasm_bindgen(getter)]
    pub fn equation_a1(&self) -> EquationA1 {
        EquationA1
    }
    
    /**
     * Equation A2 - Radiation Intensity at Receiver
     * 
     * Access to BR 187 Appendix A Equation A2 functions.
     * 
     * @memberof BR187.AppendixA
     * @returns {EquationA2} EquationA2 namespace with receiver intensity calculations
     */
    #[wasm_bindgen(getter)]
    pub fn equation_a2(&self) -> EquationA2 {
        EquationA2
    }
    
    /**
     * Equation A3 - Configuration Factor (Parallel, Centre Aligned)
     * 
     * Access to BR 187 Appendix A Equation A3 functions.
     * 
     * @memberof BR187.AppendixA
     * @returns {EquationA3} EquationA3 namespace with configuration factor calculations
     */
    #[wasm_bindgen(getter)]
    pub fn equation_a3(&self) -> EquationA3 {
        EquationA3
    }
    
    /**
     * Equation A4 - Configuration Factor (Parallel, Corner Aligned)
     * 
     * Access to BR 187 Appendix A Equation A4 functions.
     * 
     * @memberof BR187.AppendixA
     * @returns {EquationA4} EquationA4 namespace with configuration factor calculations
     */
    #[wasm_bindgen(getter)]
    pub fn equation_a4(&self) -> EquationA4 {
        EquationA4
    }
    
    /**
     * Equation A5 - Configuration Factor (Perpendicular, Corner Aligned)
     * 
     * Access to BR 187 Appendix A Equation A5 functions.
     * 
     * @memberof BR187.AppendixA
     * @returns {EquationA5} EquationA5 namespace with configuration factor calculations
     */
    #[wasm_bindgen(getter)]
    pub fn equation_a5(&self) -> EquationA5 {
        EquationA5
    }
}

/**
 * BR 187 Appendix A Equation A1 - Stefan-Boltzmann Radiation Intensity
 * 
 * @memberof BR187.AppendixA
 * @namespace EquationA1
 */
#[wasm_bindgen]
pub struct EquationA1;

#[wasm_bindgen]
impl EquationA1 {
    /**
     * Calculate radiation intensity from a fire source using Stefan-Boltzmann law
     * 
     * Formula: I = σ ε T⁴
     * 
     * @memberof BR187.AppendixA.EquationA1
     * @param {number} sigma - Stefan-Boltzmann constant (kW/m²K⁴)
     * @param {number} emissivity - Surface emissivity (dimensionless, 0-1)
     * @param {number} temperature - Absolute temperature (K)
     * @returns {number} Radiation intensity (kW/m²)
     * 
     * @example
     * ```javascript
     * const br187 = get_br_187();
     * const intensity = br187.appendix_a.equation_a1.radiation_intensity(5.67e-11, 0.9, 1273.15);
     * console.log(`Radiation intensity: ${intensity} kW/m²`);
     * ```
     */
    #[wasm_bindgen]
    pub fn radiation_intensity(&self, sigma: f64, emissivity: f64, temperature: f64) -> f64 {
        br_187::appendix_a::equation_a1::radiation_intensity(sigma, emissivity, temperature)
    }
}

/**
 * BR 187 Appendix A Equation A2 - Radiation Intensity at Receiver
 * 
 * @memberof BR187.AppendixA
 * @namespace EquationA2
 */
#[wasm_bindgen]
pub struct EquationA2;

#[wasm_bindgen]
impl EquationA2 {
    /**
     * Calculate radiation intensity at receiver location
     * 
     * Formula: q = φ × Is
     * 
     * @memberof BR187.AppendixA.EquationA2
     * @param {number} phi - Configuration factor (dimensionless)
     * @param {number} i_s - Radiation intensity at emitter (kW/m²)
     * @returns {number} Radiation intensity at receiver (kW/m²)
     * 
     * @example
     * ```javascript
     * const br187 = get_br_187();
     * const intensity = br187.appendix_a.equation_a2.radiation_intensity_at_receiver(0.5, 100.0);
     * console.log(`Intensity at receiver: ${intensity} kW/m²`);
     * ```
     */
    #[wasm_bindgen]
    pub fn radiation_intensity_at_receiver(&self, phi: f64, i_s: f64) -> f64 {
        br_187::appendix_a::equation_a2::radiation_intensity_at_receiver(phi, i_s)
    }
}

/**
 * BR 187 Appendix A Equation A3 - Configuration Factor (Parallel, Centre Aligned)
 * 
 * @memberof BR187.AppendixA
 * @namespace EquationA3
 */
#[wasm_bindgen]
pub struct EquationA3;

#[wasm_bindgen]
impl EquationA3 {
    /**
     * Calculate the X parameter for configuration factor calculations
     * 
     * Formula: X = w / (2s)
     * 
     * @memberof BR187.AppendixA.EquationA3
     * @param {number} w - Width dimension (m)
     * @param {number} s - Separation distance (m)
     * @returns {number} X parameter (dimensionless)
     */
    #[wasm_bindgen]
    pub fn x(&self, w: f64, s: f64) -> f64 {
        br_187::appendix_a::equation_a3::x(w, s)
    }
    
    /**
     * Calculate the Y parameter for configuration factor calculations
     * 
     * Formula: Y = h / (2s)
     * 
     * @memberof BR187.AppendixA.EquationA3
     * @param {number} h - Height dimension (m)
     * @param {number} s - Separation distance (m)
     * @returns {number} Y parameter (dimensionless)
     */
    #[wasm_bindgen]
    pub fn y(&self, h: f64, s: f64) -> f64 {
        br_187::appendix_a::equation_a3::y(h, s)
    }
    
    /**
     * Calculate the configuration factor for parallel centre-aligned surfaces
     * 
     * @memberof BR187.AppendixA.EquationA3
     * @param {number} x - X parameter (dimensionless)
     * @param {number} y - Y parameter (dimensionless)
     * @param {boolean} additive - Whether to use additive form (true) or subtractive (false)
     * @returns {number} Configuration factor (dimensionless)
     * 
     * @example
     * ```javascript
     * const br187 = get_br_187();
     * const x = br187.appendix_a.equation_a3.x(3.0, 7.5);
     * const y = br187.appendix_a.equation_a3.y(1.5, 7.5);
     * const phi = br187.appendix_a.equation_a3.phi(x, y, true);
     * console.log(`Configuration factor: ${phi}`);
     * ```
     */
    #[wasm_bindgen]
    pub fn phi(&self, x: f64, y: f64, additive: bool) -> f64 {
        br_187::appendix_a::equation_a3::phi(x, y, additive)
    }
}

/**
 * BR 187 Appendix A Equation A4 - Configuration Factor (Parallel, Corner Aligned)
 * 
 * @memberof BR187.AppendixA
 * @namespace EquationA4
 */
#[wasm_bindgen]
pub struct EquationA4;

#[wasm_bindgen]
impl EquationA4 {
    /**
     * Calculate the X parameter for configuration factor calculations (A4 variant)
     * 
     * Formula: X = w / s
     * 
     * @memberof BR187.AppendixA.EquationA4
     * @param {number} w - Width dimension (m)
     * @param {number} s - Separation distance (m)
     * @returns {number} X parameter (dimensionless)
     */
    #[wasm_bindgen]
    pub fn x(&self, w: f64, s: f64) -> f64 {
        br_187::appendix_a::equation_a4::x(w, s)
    }
    
    /**
     * Calculate the Y parameter for configuration factor calculations (A4 variant)
     * 
     * Formula: Y = h / s
     * 
     * @memberof BR187.AppendixA.EquationA4
     * @param {number} h - Height dimension (m)
     * @param {number} s - Separation distance (m)
     * @returns {number} Y parameter (dimensionless)
     */
    #[wasm_bindgen]
    pub fn y(&self, h: f64, s: f64) -> f64 {
        br_187::appendix_a::equation_a4::y(h, s)
    }
    
    /**
     * Calculate the configuration factor for parallel corner-aligned surfaces
     * 
     * @memberof BR187.AppendixA.EquationA4
     * @param {number} x - X parameter (dimensionless)
     * @param {number} y - Y parameter (dimensionless)
     * @param {boolean} additive - Whether to use additive form (true) or subtractive (false)
     * @returns {number} Configuration factor (dimensionless)
     * 
     * @example
     * ```javascript
     * const br187 = get_br_187();
     * const x = br187.appendix_a.equation_a4.x(3.0, 7.5);
     * const y = br187.appendix_a.equation_a4.y(1.5, 7.5);
     * const phi = br187.appendix_a.equation_a4.phi(x, y, true);
     * console.log(`Configuration factor: ${phi}`);
     * ```
     */
    #[wasm_bindgen]
    pub fn phi(&self, x: f64, y: f64, additive: bool) -> f64 {
        br_187::appendix_a::equation_a4::phi(x, y, additive)
    }
}

/**
 * BR 187 Appendix A Equation A5 - Configuration Factor (Perpendicular, Corner Aligned)
 * 
 * @memberof BR187.AppendixA
 * @namespace EquationA5
 */
#[wasm_bindgen]
pub struct EquationA5;

#[wasm_bindgen]
impl EquationA5 {
    /**
     * Calculate the X parameter for configuration factor calculations (A5 variant)
     * 
     * Formula: X = w / s
     * 
     * @memberof BR187.AppendixA.EquationA5
     * @param {number} w - Width dimension (m)
     * @param {number} s - Separation distance (m)
     * @returns {number} X parameter (dimensionless)
     */
    #[wasm_bindgen]
    pub fn x(&self, w: f64, s: f64) -> f64 {
        br_187::appendix_a::equation_a5::x(w, s)
    }
    
    /**
     * Calculate the Y parameter for configuration factor calculations (A5 variant)
     * 
     * Formula: Y = h / s
     * 
     * @memberof BR187.AppendixA.EquationA5
     * @param {number} h - Height dimension (m)
     * @param {number} s - Separation distance (m)
     * @returns {number} Y parameter (dimensionless)
     */
    #[wasm_bindgen]
    pub fn y(&self, h: f64, s: f64) -> f64 {
        br_187::appendix_a::equation_a5::y(h, s)
    }
    
    /**
     * Calculate the configuration factor for perpendicular corner-aligned surfaces
     * 
     * @memberof BR187.AppendixA.EquationA5
     * @param {number} x - X parameter (dimensionless)
     * @param {number} y - Y parameter (dimensionless)
     * @param {boolean} additive - Whether to use additive form (true) or subtractive (false)
     * @returns {number} Configuration factor (dimensionless)
     * 
     * @example
     * ```javascript
     * const br187 = get_br_187();
     * const x = br187.appendix_a.equation_a5.x(3.0, 5.0);
     * const y = br187.appendix_a.equation_a5.y(2.5, 5.0);
     * const phi = br187.appendix_a.equation_a5.phi(x, y, true);
     * console.log(`Configuration factor: ${phi}`);
     * ```
     */
    #[wasm_bindgen]
    pub fn phi(&self, x: f64, y: f64, additive: bool) -> f64 {
        br_187::appendix_a::equation_a5::phi(x, y, additive)
    }
}