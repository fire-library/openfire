pub fn areas_of_openings(openings_dimensions: Vec<(f64, f64)>) -> Vec<f64> {
    return openings_dimensions.iter().map(|(w, h)| w * h).collect()
}

pub fn equivalent_area_of_openings(areas_of_openings: Vec<f64>) -> f64 {
    return areas_of_openings.iter().sum()
}

pub fn equation_equivalent_area_of_openings(a_o: String, areas_of_openings: Vec<String>) -> String {
    let formatted_areas = areas_of_openings.join(" + ");
    format!("{} = {}", a_o, formatted_areas)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equivalent_area_of_openings() {
        let areas_of_openings = vec![1.2, 3.4, 5.6];
        let result = sum_areas(areas_of_openings);
        assert!((result - 10.2).abs() < 1e-10);
    }

    #[test]
    fn test_areas_of_openings() {
        let openings_dimensions = vec![(2.0, 3.0), (1.5, 4.0), (5.0, 0.5)];
        let result = areas_of_openings(openings_dimensions);
        assert_eq!(result, vec![6.0, 6.0, 2.5]);
    }

}
