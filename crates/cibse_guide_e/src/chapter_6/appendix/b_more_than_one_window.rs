pub fn areas_of_openings(openings_dimensions: Vec<(f64, f64)>) -> Vec<f64> {
    return openings_dimensions.iter().map(|(w, h)| w * h).collect();
}

pub fn total_area_of_openings(areas_of_openings: Vec<f64>) -> f64 {
    return areas_of_openings.iter().sum();
}

pub fn total_area_of_openings_equation(a_o: String, areas_of_openings: Vec<String>) -> String {
    let formatted_areas = areas_of_openings.join(" + ");
    return format!("{} = {}", a_o, formatted_areas)
}

pub fn equivalent_width_of_openings(widths_of_openings: Vec<f64>) -> f64 {
    return widths_of_openings.iter().sum();
}

pub fn equivalent_width_of_openings_equation(
    w_o: String,
    widths_of_openings: Vec<String>,
) -> String {
    let formatted_widths = widths_of_openings.join(" + ");
    return format!("{} = {}", w_o, formatted_widths)
}

pub fn equivalent_height_of_openings(
    equivalent_area_of_openings: f64,
    equivalent_width_of_openings: f64,
) -> f64 {
    if equivalent_width_of_openings == 0.0 {
        return 0.0; // Avoid division by zero
    }
    return equivalent_area_of_openings / equivalent_width_of_openings;
}

pub fn equivalent_height_of_openings_equation(h_o: String, a_o: String, w_o: String) -> String {
    return format!("{} = {} / {}", h_o, a_o, w_o)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_areas_of_openings() {
        let openings_dimensions = vec![(2.0, 3.0), (1.5, 4.0), (5.0, 0.5)];
        let result = areas_of_openings(openings_dimensions);
        assert_eq!(result, vec![6.0, 6.0, 2.5]);
    }

    #[test]
    fn test_total_area_of_openings() {
        let areas_of_openings = vec![1.2, 3.4, 5.6];
        let result = total_area_of_openings(areas_of_openings);
        assert!((result - 10.2).abs() < 1e-10);
    }

    #[test]
    fn test_equivalent_width_of_openings() {
        let widths_of_openings = vec![2.0, 3.0, 5.0];
        let result = equivalent_width_of_openings(widths_of_openings);
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
