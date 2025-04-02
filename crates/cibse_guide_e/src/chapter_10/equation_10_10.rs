pub mod openfire_runner;

pub fn limiting_velocity_10_10(g: f64, h: f64, t_f: f64, t_0: f64) -> f64 {
    return 0.64 * (g * H  * (t_f - t_0)/t_f).powf(0.5);
}


pub fn limiting_velocity_10_11(q: f64, z: f64) -> f64 {
    return 0.057 * (q / z).powf(1.0/3.0);
}


pub fn limiting_velocity_10_12(k: f64, g: f64, q: f64, w: f64, rho: f64, c: f64, t: f64) -> f64 {
    return k * ((g*q) / (w * rho * c * t)).powf(1.0/3.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation_10_10() {
        let result = limiting_velocity_10_10(9.8, 2.2, 973.0, 293.0);
        assert_eq!(result, 2.52310759628797);
    }

    #[test]
    fn test_equation_10_11() {
        let result = limiting_velocity_10_11(1000.0, 1.5);
        assert_eq!(result,0.49794086489969 );
    }

    #[test]
    fn test_equation_10_12() {
        let result = limiting_velocity_10_12(1.0, 1000.0, 9.8, 2.5, 1.2, 1.0, 773.0);
        assert_eq!(result, 1.22592912182919);
    }
}
