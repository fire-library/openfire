#[path ="b_more_than_one_window.rs"]
pub mod b_more_than_one_window;

pub fn total_area_of_openings_per_wall(dimensions_of_openings_wall1: Vec<(f64, f64)>) -> f64 {
    let vector_of_areas_wall1 = b_more_than_one_window::areas_of_openings(dimensions_of_openings_wall1);
    return b_more_than_one_window::total_area_of_openings(vector_of_areas_wall1);
}

pub fn total_area_of_openings_per_wall_equation(a_o: String, areas_of_openings: Vec<String>) -> String {
    return b_more_than_one_window::total_area_of_openings_equation(a_o, areas_of_openings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_area_of_openings() {
        let dimension_openings_wall1 = vec![(2.0, 3.0), (1.5, 4.0), (5.0, 0.5)];
        let  result = total_area_of_openings_per_wall(dimension_openings_wall1);
        assert_eq!(result, 14.5);
    }
}
