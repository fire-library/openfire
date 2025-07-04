pub fn floor_area(w1: f64, w2: f64) -> f64 {
    return w1 * w2;
}

pub fn floor_area_equation(a_f: String, w1: String, w2: String) -> String {
    return format!("{} = {} \\cdot {}", a_f, w1, w2,);
}

pub fn area_of_opening(wo: f64, ho: f64) -> f64 {
    return wo * ho;
}

pub fn area_of_opening_equation(a_o: String, w_o: String, h_o: String) -> String {
    return format!("{} = {} \\cdot {}", a_o, w_o, h_o,);
}

pub fn internal_surface_area(a_f: f64, h: f64, w1: f64, w2: f64, a_o: f64) -> f64 {
    return 2.0 * a_f + 2.0 * h * (w1 + w2) - a_o;
}

pub fn internal_surface_area_equation(
    a_net: String,
    a_f: String,
    h: String,
    w1: String,
    w2: String,
    a_o: String,
) -> String {
    return format!(
        "{} = 2 \\cdot {} + 2 \\cdot {} ({} + {}) - {}",
        a_net, a_f, h, w1, w2, a_o
    );
}

pub fn ratio_depth_over_height(w1: f64, w2: f64) -> f64 {
    return w2 / w1;
}

pub fn ratio_depth_over_height_equation(d: String, w: String, w1: String, w2: String) -> String {
    return format!("{} / {} = {} / {}", d, w, w2, w1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_area() {
        let result = floor_area(4.5, 3.5);
        assert_eq!(result, 15.75);
    }

    #[test]
    fn test_area_of_opening() {
        let result = area_of_opening(0.9, 2.1);
        assert_eq!(result, 1.8900000000000001);
    }

    #[test]
    fn test_internal_surface_area() {
        let result = internal_surface_area(15.75, 3.0, 4.5, 3.5, 1.89);
        assert_eq!(result, 77.61);
    }

    #[test]
    fn test_ratio_depth_over_height() {
        let result = ratio_depth_over_height(4.5, 3.5);
        assert_eq!(result, 0.7777777777777778)
    }
}
