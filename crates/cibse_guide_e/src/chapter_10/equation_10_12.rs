pub mod openfire_runner;

pub fn limiting_velocity(k: f64, g: f64, q: f64, w: f64, rho: f64, c: f64, t: f64) -> f64 {
    return k * ((g*q) / (w * rho * c * t)).powf(1.0/3.0);
}

fn limiting_velocity_symbols(v_e: String, k: String, g: String, q: String, w: String, rho: String, c: String, t: String) -> String {
    format!(
        "{} = {} \\space \\frac{{{} \\space {}}}{{{} \\space {} \\space {} \\space {}}} ^ {{1/3}}",
        v_e, k, g, q, w, rho, c, t,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = limiting_velocity(1.0, 1000.0, 9.8, 2.5, 1.2, 1.0, 773.0);
        assert_eq!(result, 1.22592912182919);
    }
}
