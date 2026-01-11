pub fn fed(c_i: Vec<f64>, delta_t_i: Vec<f64>, lc_t50: f64) -> f64 {
    let numerator: f64 = c_i.iter().zip(delta_t_i.iter()).map(|(c, dt)| c * dt).sum();
    numerator / lc_t50
}

#[cfg(not(coverage))]
pub fn fed_equation(
    fed: String,
    c_i: String,
    delta_t_i: String,
    lc_t50: String,
) -> String {
    format!(
        "{} = \\frac{{ \\sum {} \\times {} }}{{ {} }}",
        fed, c_i, delta_t_i, lc_t50
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fed() {
        let c_i = vec![0.1, 0.2, 0.3];
        let delta_t_i = vec![1.0, 2.0, 3.0];
        let lc_t50 = 10.0;
        let result = fed(c_i, delta_t_i, lc_t50);
        let expected = (0.1 * 1.0 + 0.2 * 2.0 + 0.3 * 3.0) / 10.0; // (0.1 + 0.4 + 0.9) / 10.0 = 0.14
        assert!((result - expected).abs() < 1e-6);
    }
}