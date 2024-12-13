//! This file contains the implementation of the equation 10-18a and 10-18b.
//! Both equations are identical. The only difference is the range that
//! defines the regime of the fire.

//! Assumptions
//! rho = 9.81 Density of the gas (kg/m^3)

pub fn calculate(rho: f64, a_w: f64, h: f64, a_f: f64) -> f64 {
    let g: f64 = 9.81;

    let numerator = rho * g.powf(0.5) * a_w * h.powf(0.5);

    return numerator / a_f;
}

#[derive(Debug, PartialEq)]
pub enum HeatingRegime {
    VentilationControlled,
    FuelControlled,
    Crossover,
}
pub fn heating_regime(number: f64) -> HeatingRegime {
    if number < 0.235 {
        return HeatingRegime::VentilationControlled;
    } else if number > 0.290 {
        return HeatingRegime::FuelControlled;
    } else {
        return HeatingRegime::Crossover;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero() {
        let result = calculate(0.0, 0.0, 0.0, 1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_positive() {
        let result = calculate(1.0, 2.0, 9.0, 1.0);

        println!("{}", result);
        assert!((result - 18.792551716038993).abs() < f64::EPSILON);
    }

    #[test]
    fn test_heating_regime_ventilation_controlled() {
        let result = heating_regime(0.234);
        assert_eq!(result, HeatingRegime::VentilationControlled);
    }

    #[test]
    fn test_heating_regime_fuel_controlled() {
        let result = heating_regime(0.291);
        assert_eq!(result, HeatingRegime::FuelControlled);
    }

    #[test]
    fn test_heating_regime_crossover() {
        let result = heating_regime(0.236);
        assert_eq!(result, HeatingRegime::Crossover);
    }
}
