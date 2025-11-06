pub use super::common;

pub fn area_of_floor(w1: f64, w2: f64) -> f64 {
    common::area_of_floor(w1, w2)
}

pub fn area_of_floor_equation(a_f: String, w1: String, w2: String) -> String {
    common::area_of_floor_equation(a_f, w1, w2)
}

pub fn sum_area_of_openings_per_wall(dimensions_of_openings_wall_per_wall: Vec<(f64, f64)>) -> f64 {
    let vector_of_areas_wall1 =
        common::areas_of_openings_multiple_openings(dimensions_of_openings_wall_per_wall);
    common::sum_areas_of_openings(vector_of_areas_wall1)
}

pub fn sum_area_of_openings_per_wall_equation(
    a_o: String,
    areas_of_openings: Vec<String>,
) -> String {
    common::sum_areas_of_openings_equation(a_o, areas_of_openings)
}

pub fn sum_area_of_openigs(areas_of_openings: Vec<f64>) -> f64 {
    common::sum_areas_of_openings(areas_of_openings)
}

pub fn sum_area_of_openings_equation(a_o: String, areas_of_openings: Vec<String>) -> String {
    common::sum_areas_of_openings_equation(a_o, areas_of_openings)
}

pub fn ratio_depth_over_height(w1: f64, w2: f64, ao_w1: f64, ao: f64) -> f64 {
    (w2 / w1) * (ao_w1 / ao)
}

pub fn ratio_depth_over_height_equation(
    d: String,
    w: String,
    w1: String,
    w2: String,
    ao_w1: String,
    ao: String,
) -> String {
    format!(
        "{} / {} = ({} / {}) \\cdot ({} / {})",
        d, w, w2, w1, ao_w1, ao
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratio_depth_over_height() {
        let w1 = 2.0;
        let w2 = 4.0;
        let ao_w1 = 6.0;
        let ao = 12.0;
        let result = ratio_depth_over_height(w1, w2, ao_w1, ao);
        assert!((result - 1.0).abs() < 1e-10);
    }
}
