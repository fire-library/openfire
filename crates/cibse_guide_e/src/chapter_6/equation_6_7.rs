pub mod openfire_runner;

pub fn heat_release_rate_flashover(a_vo: f64, h_o: f64) -> f64 {
    return 600.0 * a_vo * h_o.powf(0.5);
}

pub fn equation(q_f: String, a_vo: String, h_o: String) -> String {
    format!("{} = 600 \\space {} \\sqrt {{{}}}", q_f, a_vo, h_o,)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = heat_release_rate_flashover(2.0, 2.1);
        assert_eq!(result, 1738.96520954273);
    }
}
