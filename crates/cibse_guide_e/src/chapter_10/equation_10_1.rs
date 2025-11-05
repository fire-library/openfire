pub fn max_volumetric_flow_rate(gamma: f64, d: f64, t_s: f64, t_0: f64) -> f64 {
    let left = 4.16 * gamma * d.powf(5.0 / 2.0);
    let right = ((t_s - t_0) / t_0).powf(0.5);

    let v_max = left * right;

    v_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation() {
        let result = max_volumetric_flow_rate(0.5, 1.5, 300.0, 290.0);
        assert_eq!(result, 1.0643696531847804);
    }
}
