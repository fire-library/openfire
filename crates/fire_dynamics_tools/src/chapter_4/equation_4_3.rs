pub fn corner_fire_flame_height(q: f64) -> f64 {
    0.075 * q.powf(3.0 / 5.0)
}

pub fn corner_fire_flame_height_equation(h_f: String, q: String) -> String {
    format!("{} = 0.075 \\cdot {}^{{\\frac{{3}}{{5}}}}", h_f, q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_fire_flame_height() {
        let q = 700.0;
        let expected = 3.820498974099;

        let result = corner_fire_flame_height(q);

        assert!(
            (result - expected).abs() < 1e-4,
            "For input {}, result should be approximately {}, but got {}",
            q,
            expected,
            result
        );
    }
}
