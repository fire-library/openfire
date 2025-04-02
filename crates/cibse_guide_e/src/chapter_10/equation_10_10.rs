pub mod openfire_runner;

pub fn limiting_velocity_10_10(g: f64, H: f64, t_f: f64, t_0: f64) -> f64 {
    return 0.64 * (g * H  * (t_f - t_0)/t_f).powf(0.5);
}


pub fn limiting_velocity_10_11(k: f64, g: f64, q: f64, w: f64, rho: f64, c: f64, t: f64) -> f64 {
    return k * ((g *q) / (w * rho * c * t)).powf(1/3);
}


pub fn limiting_velocity_10_12(g: f64, H: f64, t_f: f64, t_0: f64) -> f64 {
    return 0.64 * (g * H  * (t_f - t_0)/t_f).powf(0.5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation() {
        let result = limiting_velocity_10_10(9.8, 2.2, 973.0, 293.0);
        assert_eq!(result, 2.52310759628797);
    }

    #[test]
    fn test_equation() {
        let result = limiting_velocity_10_11(9.8, 2.2, 973.0, 293.0);
        assert_eq!(result, );
    }

    #[test]
    fn test_equation() {
        let result = limiting_velocity_10_10(9.8, 2.2, 973.0, 293.0);
        assert_eq!(result, 2.52310759628797);
    }
}
