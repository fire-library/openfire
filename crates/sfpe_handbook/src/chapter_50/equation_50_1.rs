pub fn delta_p_so(t_0: f64, t_s: f64) -> f64 {
    let t_0_abs = t_0 + 273.0;
    let t_s_abs = t_s + 273.0;
    3460.0 * (1.0 / (t_0_abs) + 1.0 / (t_s_abs)) 
}

pub fn delta_p_so_equation(delta_p_so: String, t_0: String, t_s: String) -> String {
    format!("{} = 3460 * (1 / ({} + 273) + 1 / ({} + 273))", delta_p_so, t_0, t_s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_p_so() {
        let result = delta_p_so(27.0, 47.0);
        let expected = 0.0;
        assert!((result - expected).abs() < 1e-6);
    }
}