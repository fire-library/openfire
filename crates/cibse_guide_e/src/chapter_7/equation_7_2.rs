pub mod openfire_runner;

pub fn stair_capacity(w: f64, n: i8) -> i8 {
    return 200 * w + 50*(w - 0.3) * (n - 1);
}

pub fn equation(q_f: String, a_vo: String, h_o: String) -> String {
    format!("{} = 600 \\cdot {} \\sqrt {{{}}}", q_f, a_vo, h_o,)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = heat_release_rate_flashover(2.0, 2.1);
        assert_eq!(result, 1738.9652095427327);
    }
}
