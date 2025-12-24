pub fn floor_area_of_compartment_with_core(w1: f64, w2: f64, c1: f64, c2: f64) -> f64 {
    w1 * w2 - c1 * c2
}

#[cfg(not(coverage))]
pub fn floor_area_of_compartment_with_core_equation(
    a_f: String,
    w1: String,
    w2: String,
    c1: String,
    c2: String,
) -> String {
    format!("{} = {} \\cdot {} - {} \\cdot {}", a_f, w1, w2, c1, c2)
}

pub fn internal_surface_area_of_compartment_with_core(
    a_f: f64,
    h: f64,
    w1: f64,
    w2: f64,
    c1: f64,
    c2: f64,
    a_o: f64,
) -> f64 {
    2.0 * a_f + 2.0 * h * (w1 + w2 + c1 + c2) - a_o
}

#[cfg(not(coverage))]
pub fn internal_surface_area_of_compartment_with_core_equation(
    a_net: String,
    a_f: String,
    h: String,
    w1: String,
    w2: String,
    c1: String,
    c2: String,
    a_o: String,
) -> String {
    format!(
        "{} = 2 \\cdot {} + 2 \\cdot {} ({} + {} + {} + {}) - {}",
        a_net, a_f, h, w1, w2, c1, c2, a_o
    )
}

pub fn ratio_depth_over_height_compartment_with_core(
    w1: f64,
    w2: f64,
    c1: f64,
    c2: f64,
    ao: f64,
    ao_w1: f64,
) -> f64 {
    ((w2 - c2) * ao_w1) / ((w1 - c1) * ao)
}
#[cfg(not(coverage))]
pub fn ratio_depth_over_height_compartment_with_core_equation(
    d: String,
    w: String,
    w1: String,
    w2: String,
    c1: String,
    c2: String,
    ao: String,
    ao_w1: String,
) -> String {
    format!(
        "{} / {} = (({} - {}) \\cdot {}) / (({} - {}) \\cdot {})",
        d, w, w2, c2, ao_w1, w1, c1, ao
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_area_of_compartment_with_core() {
        let w1 = 10.0;
        let w2 = 8.0;
        let c1 = 2.0;
        let c2 = 3.0;
        let result = floor_area_of_compartment_with_core(w1, w2, c1, c2);
        assert_eq!(result, 74.0);
    }

    #[test]
    fn test_internal_surface_area_of_compartment_with_core() {
        let a_f = 74.0;
        let h = 3.0;
        let w1 = 10.0;
        let w2 = 8.0;
        let c1 = 2.0;
        let c2 = 3.0;
        let a_o = 5.0;
        let result = internal_surface_area_of_compartment_with_core(a_f, h, w1, w2, c1, c2, a_o);
        // Calculation: 2*74 + 2*3*(10+8+2+3) - 5 = 148 + 2*3*23 - 5 = 148 + 138 - 5 = 281
        assert_eq!(result, 281.0);
    }

    #[test]
    fn test_ratio_depth_over_height_compartment_with_core() {
        let w1 = 10.0;
        let w2 = 8.0;
        let c1 = 2.0;
        let c2 = 3.0;
        let ao = 12.0;
        let ao_w1 = 6.0;
        let result = ratio_depth_over_height_compartment_with_core(w1, w2, c1, c2, ao, ao_w1);
        // ((8-3)*6)/((10-2)*12) = (5*6)/(8*12) = 30/96 = 0.3125
        assert!((result - 0.3125).abs() < 1e-10);
    }
}
