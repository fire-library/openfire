pub fn acceptance_capacity_stair(w_e: f64, t: f64, rho: f64, a: f64, s: i32) -> i32 {
    let s = s as f64;
    let result = 1.2 * w_e * t + rho * a * (s - 1.0);
    result.floor() as i32
}

pub fn equation(n_in: String, w_e: String, t: String, p: String, a: String, s: String) -> String {
    format!(
        "{} = 1.2 \\cdot {} \\cdot {} + {} \\cdot {} \\cdot ({} - 1)",
        n_in, w_e, t, p, a, s
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = acceptance_capacity_stair(0.9, 150.0, 2.0, 10.0, 5);
        assert_eq!(result, 242);
    }
}
