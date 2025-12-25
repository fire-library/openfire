pub fn hot_gas_temperature_increase(
    q: f64,
    a_v: Vec<f64>,
    h_v: Vec<f64>,
    a_t: f64,
    h_k: f64,
) -> f64 {
    let area_av_times_hv: f64 = a_v
        .iter()
        .zip(h_v.iter())
        .map(|(av, hv)| av * hv.powf(0.5))
        .sum();

    6.85 * (q.powf(2.0) / ((area_av_times_hv) * (a_t * h_k))).powf(1.0 / 3.0)
}

#[cfg(not(coverage))]
pub fn hot_gas_temperature_increase_equation(
    delta_t_g: String,
    q: String,
    a_v: String,
    h_v: String,
    a_t: String,
    h_k: String,
) -> String {
    format!(
        "{} = 6.85 \\cdot \\left( \\frac{{{}^{{2}}}}{{\\left(\\sum_{{i}} {} \\cdot \\sqrt{{{}}}\\right) \\cdot ({} \\cdot {})}} \\right)^{{1/3}}",
        delta_t_g, q, a_v, h_v, a_t, h_k
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_gas_temperature_increase() {
        let q = 1000.0;
        let a_v = vec![2.5, 1.5];
        let h_v = vec![2.0, 1.0];
        let a_t = 75.0;
        let h_k = 0.035;
        let expected_result = 289.7114284;

        let result = hot_gas_temperature_increase(q, a_v, h_v, a_t, h_k);

        assert!((result - expected_result).abs() < 1e-6,);
    }
}
