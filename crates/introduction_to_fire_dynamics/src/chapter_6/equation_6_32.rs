use std::f64::consts::PI;

/// Conversion factor from kW/m² to W/m²
const KW_PER_M2_TO_W_PER_M2: f64 = 1000.0;

pub fn time_to_ignition_thermally_thick(k: f64, rho: f64, c: f64, temp_ig: f64, temp_o: f64, q_r: f64) -> f64 {
    return (PI / 4.0)* (k * rho * c) * (temp_ig - temp_o).powf(2.0) / (q_r * KW_PER_M2_TO_W_PER_M2).powf(2.0);

}

pub fn time_to_ignition_thermallythick_equation(t_ig: String, k: String, rho: String, c: String, temp_ig: String, temp_o: String, q_r: String) -> String {
    return format!("{} = \\frac{{\\pi}}{{4}} \\cdot {} \\cdot {} \\cdot {} \\left( {} - {} \\right)^2 \\cdot \\frac{{1}}{{( {} \\cdot 1000 )^2}}", t_ig, k, rho, c, temp_ig, temp_o, q_r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_to_ignition_thermally_thick() {
        let result = time_to_ignition_thermally_thick(0.19, 1190.0, 1420.0, 300.0, 25.0, 20.0);
        assert_eq!(result, 47.67428456490953);
    }

}
