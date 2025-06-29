pub fn floor_area(w1: f64, w2: f64) -> f64 {
    return w1 * w2
}

pub fn equation_floor_area(a_f: String, w1: String, w2: String) -> String {
    format!("{} = {} \\cdot {}", a_f, w1, w2,)
}

pub fn area_of_opening(wo: f64, ho: f64) -> f64 {
    return wo * ho;
}

pub fn equation_area_of_opening(a_o: String, w_o: String, h_o: String) -> String {
    format!("{} = {} \\cdot {}", a_o, w_o, h_o,)
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
        assert_eq!(result, 1.89);
    }
}
