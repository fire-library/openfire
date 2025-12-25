pub use super::common;

pub fn area_of_floor(w1: f64, w2: f64) -> f64 {
    common::area_of_floor(w1, w2)
}

#[cfg(not(coverage))]
pub fn area_of_floor_equation(a_f: String, w1: String, w2: String) -> String {
    common::area_of_floor_equation(a_f, w1, w2)
}

pub fn sum_area_of_openings_per_wall(dimensions_of_openings_wall_per_wall: Vec<(f64, f64)>) -> f64 {
    let vector_of_areas_wall1 =
        common::areas_of_openings_multiple_openings(dimensions_of_openings_wall_per_wall);
    common::sum_areas_of_openings(vector_of_areas_wall1)
}

#[cfg(not(coverage))]
pub fn sum_area_of_openings_per_wall_equation(
    a_o: String,
    areas_of_openings: Vec<String>,
) -> String {
    common::sum_areas_of_openings_equation(a_o, areas_of_openings)
}

pub fn sum_area_of_openigs(areas_of_openings: Vec<f64>) -> f64 {
    common::sum_areas_of_openings(areas_of_openings)
}

#[cfg(not(coverage))]
pub fn sum_area_of_openings_equation(a_o: String, areas_of_openings: Vec<String>) -> String {
    common::sum_areas_of_openings_equation(a_o, areas_of_openings)
}

pub fn ratio_depth_over_height(w1: f64, w2: f64, ao_w1: f64, ao: f64) -> f64 {
    (w2 / w1) * (ao_w1 / ao)
}

#[cfg(not(coverage))]
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
    fn test_area_of_floor() {
        let result = area_of_floor(4.5, 3.5);
        assert_eq!(result, 15.75);
    }

    #[test]
    fn test_sum_area_of_openings_per_wall() {
        let dimensions = vec![(2.0, 3.0), (1.5, 4.0), (5.0, 0.5)];
        let result = sum_area_of_openings_per_wall(dimensions);
        assert!((result - 14.5).abs() < 1e-10);
    }

    #[test]
    fn test_sum_area_of_openigs() {
        let areas_of_openings = vec![1.2, 3.4, 5.6];
        let result = sum_area_of_openigs(areas_of_openings);
        assert!((result - 10.2).abs() < 1e-10);
    }

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
