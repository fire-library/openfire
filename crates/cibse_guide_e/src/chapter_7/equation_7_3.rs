pub mod openfire_runner;

pub fn required_width_stair(p: i16, n: i16) -> f64 {
    return (p + 15*n - 15) / (150 + 50 * n);
}

pub fn equation(p: String, w: String, n: String) -> String {
    format!(
        "{} = 200 \\cdot {} + 50 \\cdot ({} - 0.3) \\cdot ({} - 1)",
        p, w, w, n
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = stair_capacity(1.2, 6);
        assert_eq!(result, 465.0);
    }
}
