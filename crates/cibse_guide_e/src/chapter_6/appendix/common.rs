pub fn area_of_floor(w1: f64, w2: f64) -> f64 {
    return w1 * w2;
}

pub fn area_of_floor_equation(a_f: String, w1: String, w2: String) -> String {
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

pub fn ratio_depth_over_width(w1: f64, w2: f64) -> f64 {
    return w2 / w1;
}

pub fn ratio_depth_over_width_equation(d_over_w: String, w1: String, w2: String) -> String {
    return format!("{} = {} / {}", d_over_w, w2, w1);
}

pub fn areas_of_openings_multiple_openings(openings_dimensions: Vec<(f64, f64)>) -> Vec<f64> {
    return openings_dimensions
        .iter()
        .map(|(w, h)| area_of_opening(*w, *h))
        .collect();
}

pub fn sum_areas_of_openings(areas_of_openings: Vec<f64>) -> f64 {
    return areas_of_openings.iter().sum();
}

pub fn sum_areas_of_openings_equation(a_o: String, areas_of_openings: Vec<String>) -> String {
    let formatted_areas = areas_of_openings.join(" + ");
    return format!("{} = {}", a_o, formatted_areas);
}

pub fn sum_width_of_compartment_openings(widths_of_openings: Vec<f64>) -> f64 {
    return widths_of_openings.iter().sum();
}

pub fn sum_width_of_compartment_openings_equation(
    w_o: String,
    widths_of_openings: Vec<String>,
) -> String {
    let formatted_widths = widths_of_openings.join(" + ");
    return format!("{} = {}", w_o, formatted_widths);
}

pub fn equivalent_height_for_compartment_openings(
    equivalent_area_of_openings: f64,
    equivalent_width_of_openings: f64,
) -> f64 {
    return equivalent_area_of_openings / equivalent_width_of_openings;
}

pub fn equivalent_height_for_compartment_openings_equation(
    h_o: String,
    a_o: String,
    w_o: String,
) -> String {
    return format!("{} = {} / {}", h_o, a_o, w_o);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_of_floor() {
        let result = area_of_floor(4.5, 3.5);
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
    fn test_ratio_depth_over_width() {
        let result = ratio_depth_over_width(4.5, 3.5);
        assert_eq!(result, 0.7777777777777778)
    }

    #[test]
    fn test_areas_of_openings_multiple_openings() {
        let openings_dimensions = vec![(2.0, 3.0), (1.5, 4.0), (5.0, 0.5)];
        let result = areas_of_openings_multiple_openings(openings_dimensions);
        assert_eq!(result, vec![6.0, 6.0, 2.5]);
    }

    #[test]
    fn test_sum_areas_of_openings() {
        let areas_of_openings = vec![1.2, 3.4, 5.6];
        let result = sum_areas_of_openings(areas_of_openings);
        assert!((result - 10.2).abs() < 1e-10);
    }

    #[test]
    fn test_sum_width_of_compartment_openings() {
        let widths_of_openings = vec![2.0, 3.0, 5.0];
        let result = sum_width_of_compartment_openings(widths_of_openings);
        assert!((result - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_equivalent_height_for_compartment_openings() {
        let equivalent_area = 10.0;
        let equivalent_width = 2.0;
        let result = equivalent_height_for_compartment_openings(equivalent_area, equivalent_width);
        assert!((result - 5.0).abs() < 1e-10);
    }
}
