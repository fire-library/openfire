pub fn wall_flame_height_correlation(q: f64) -> f64 {
    0.034 * q.powf(2.0 / 3.0)
}

pub fn wall_flame_height_correlation_equation(h_f: String, q: String) -> String {
    format!("{} = 0.034 \\cdot {}^{{\\frac{{2}}{{3}}}}", h_f, q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_flame_height_correlation() {
        let q = 700.0;
        let expected = 2.680469955456;

        let result = wall_flame_height_correlation(q);
        
        assert!(
            (result - expected).abs() < 1e-4,
            "For input {}, result should be approximately {}, but got {}",
            q,
            expected,
            result
        );
    }
}
