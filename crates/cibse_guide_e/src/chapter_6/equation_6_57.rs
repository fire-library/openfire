pub fn height_of_flame_aboveopening(r: f64, w: f64, h_o: f64) -> f64 {
    12.8 * (r / w).powf(2.0 / 3.0) - h_o
}

#[cfg(not(coverage))]
pub fn equation(z_fo: String, r: String, w: String, h_o: String) -> String {
    format!("{} = 12.8 \\cdot ({} / {})^{{(2/3)}} - {}", z_fo, r, w, h_o,)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = height_of_flame_aboveopening(0.2, 1.0, 2.1);
        assert_eq!(result, 2.2775384234923455);
    }
}
