pub use super::common;

pub fn area_of_floor(w1: f64, w2: f64) -> f64 {
    return common::area_of_floor(w1, w2);
}

pub fn area_of_floor_equation(a_f: String, w1: String, w2: String) -> String {
    return common::area_of_floor_equation(a_f, w1, w2);
}

pub fn areas_of_openings_multiple_openings(openings_dimensions: Vec<(f64, f64)>) -> Vec<f64> {
    return common::areas_of_openings_multiple_openings(openings_dimensions);
}

pub fn sum_areas_of_openings(areas_of_openings: Vec<f64>) -> f64 {
    return common::sum_areas_of_openings(areas_of_openings);
}

pub fn sum_area_of_openings_equation(a_o: String, areas_of_openings: Vec<String>) -> String {
    return common::sum_areas_of_openings_equation(a_o, areas_of_openings);
}

pub fn sum_width_of_compartment_openings(widths_of_openings: Vec<f64>) -> f64 {
    return common::sum_width_of_compartment_openings(widths_of_openings);
}

pub fn equivalent_width_of_openings_equation(
    w_o: String,
    widths_of_openings: Vec<String>,
) -> String {
    let formatted_widths = widths_of_openings.join(" + ");
    return format!("{} = {}", w_o, formatted_widths);
}

pub fn equivalent_height_of_openings(
    equivalent_area_of_openings: f64,
    equivalent_width_of_openings: f64,
) -> f64 {
    return equivalent_area_of_openings / equivalent_width_of_openings;
}

pub fn equivalent_height_of_openings_equation(h_o: String, a_o: String, w_o: String) -> String {
    return format!("{} = {} / {}", h_o, a_o, w_o);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_width_of_compartment_openings() {
        let widths_of_openings = vec![2.0, 3.0, 5.0];
        let result = sum_width_of_compartment_openings(widths_of_openings);
        assert!((result - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_equivalent_height_of_openings() {
        let equivalent_area = 10.0;
        let equivalent_width = 2.0;
        let result = equivalent_height_of_openings(equivalent_area, equivalent_width);
        assert!((result - 5.0).abs() < 1e-10);
    }
}
