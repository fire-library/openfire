pub fn pressure_difference_stack_effect(t_0: f64, t_s: f64, z: f64) -> f64 {
    let t_0_abs = t_0 + 273.0;
    let t_s_abs = t_s + 273.0;
    3460.0 * (1.0 / (t_0_abs) - 1.0 / (t_s_abs)) * z 
}

pub fn pressure_difference_stack_effect_equation(delta_p_so: String, t_0: String, t_s: String, z: String) -> String {
    format!("{} = 3460 * (1 / ({} + 273) - 1 / ({} + 273)) * {}", delta_p_so, t_0, t_s, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_difference_stack_effect() {
        let result = pressure_difference_stack_effect(-18.0, 21.0, 30.0);
        let expected = 53.99759904;
        assert!((result - expected).abs() < 1e-6);
    }
}