pub fn line_fire_flame_height(q: f64) -> f64 {
    0.017 * q.powf(2.0 / 3.0)
}

pub fn line_fire_flame_height_equation(h_f: String, q: String) -> String {
    format!("{} = 0.017 \\cdot {}^{{\\frac{{2}}{{3}}}}", h_f, q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_fire_flame_height() {
        let q = 700.0;
        let expected = 1.340234977728;

        let result = line_fire_flame_height(q);

        assert!(
            (result - expected).abs() < 1e-4,
            "For input {}, result should be approximately {}, but got {}",
            q,
            expected,
            result
        );
    }
}
