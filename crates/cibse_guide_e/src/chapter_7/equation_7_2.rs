pub fn stair_capacity(w: f64, n: i32) -> i32 {
    let result: f64 = 200.0 * w + 50.0 * (w - 0.3) * (n as f64 - 1.0);
    return result.floor() as i32;
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
        assert_eq!(result, 465);
    }
}
