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

pub fn sum_width_of_compartment_openings_equation(
    w_o: String,
    widths_of_openings: Vec<String>,
) -> String {
    return common::sum_width_of_compartment_openings_equation(w_o, widths_of_openings);
}

pub fn equivalent_height_for_compartment_openings(
    equivalent_area_of_openings: f64,
    equivalent_width_of_openings: f64,
) -> f64 {
    return common::equivalent_height_for_compartment_openings(
        equivalent_area_of_openings,
        equivalent_width_of_openings,
    );
}

pub fn equivalent_height_for_compartment_openings_equation(
    h_o: String,
    a_o: String,
    w_o: String,
) -> String {
    return common::equivalent_height_for_compartment_openings_equation(h_o, a_o, w_o);
}
