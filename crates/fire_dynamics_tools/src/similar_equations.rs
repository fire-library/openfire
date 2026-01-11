/// Similar equations grouped by functionality across different fire engineering standards.
/// This module provides cross-references to equations that calculate similar parameters
/// but may appear in different standards or use slightly different approaches.

/// Fractional Effective Dose (FED) calculations
/// 
/// These equations calculate the fractional effective dose, which represents
/// the accumulated toxic exposure over time as a fraction of the lethal dose.
pub mod fractional_effective_dose {
    /// SFPE Handbook Chapter 50, Equation 50-18
    /// 
    /// Calculates FED using sum of concentration × time intervals divided by lethal concentration.
    /// Formula: FED = Σ(c_i × Δt_i) / LC_t50
    /// 
    /// This implementation handles discrete time intervals with varying concentrations.
    pub use sfpe_handbook::chapter_50::equation_50_18::fed as sfpe_fed;
    
    /// CIBSE Guide E Chapter 10, Equation 10-8  
    ///
    /// Calculates FED using mass flow rate × time divided by lethal concentration.
    /// Formula: FED = (m_f × t) / LC_50
    /// 
    /// This implementation assumes constant mass flow rate over the exposure time.
    pub use cibse_guide_e::chapter_10::equation_10_8::fractional_effective_dose as cibse_fed;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_sfpe_fed() {
            let c_i = vec![0.1, 0.2, 0.3];
            let delta_t_i = vec![1.0, 2.0, 3.0];
            let lc_t50 = 10.0;
            let result = sfpe_fed(c_i, delta_t_i, lc_t50);
            let expected = 0.14; // (0.1*1.0 + 0.2*2.0 + 0.3*3.0) / 10.0
            assert!((result - expected).abs() < 1e-6);
        }

        #[test]
        fn test_cibse_fed() {
            let result = cibse_fed(2.0, 120.0, 1000.0);
            assert_eq!(result, 0.24);
        }
    }
}