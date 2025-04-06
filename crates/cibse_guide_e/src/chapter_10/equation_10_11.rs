pub mod openfire_runner;

pub fn limiting_velocity(q: f64, z: f64) -> f64 {
    return 0.057 * (q / z).powf(1.0/3.0);
}

pub fn equation(v_e: String, q: String, z: String) -> String {
    format!(
        "{} = 0.057 \\space \\frac{{{}}}{{{}}} ^ {{1/3}}",
        v_e, q, z,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = limiting_velocity(1000.0, 1.5);
        assert_eq!(result,0.49794086489969036);
    }
}
