pub mod openfire_runner;

pub fn af(w_1: f64, w_2: f64) -> f64 {
    return w_1 * w_2;
}

pub fn ao(w_o: f64, h_o: f64) -> f64 {
    return w_o * h_o;
}

pub fn anet(a_f: f64, h: f64, w_1: f64, w_2: f64, a_o: f64) -> f64 {
    return 2.0 * a_f + 2.0 * h * (w_1 + w_2) - a_o;
}

pub fn d_over_w(w_1: f64, w_2: f64) -> f64 {
    return w_2 / w_1;
}

pub fn equation_af(a_f: String, w1: String, w2: String) -> String {
    format!("{} = {} \\cdot {}", a_f, w1, w2)
}

pub fn equation_ao(a_o: String, w_o: String, h_o: String) -> String {
    format!("{} = {} \\cdot {}", a_o, w_o, h_o)
}

pub fn equation_anet(
    a_net: String,
    a_f: String,
    h: String,
    w1: String,
    w2: String,
    a_o: String,
) -> String {
    format!(
        "{} = 2 * {} + 2 * {} ({} + {}) - {}",
        a_net, a_f, h, w1, w2, a_o
    )
}

pub fn equation_doverw(d_over_w: String, w_1: String, w_2: String) -> String {
    format!("{} = {} / {}", d_over_w, w_2, w_1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_af() {
        let result = af(3.5, 5.0);
        assert_eq!(result, 17.5);
    }

    #[test]
    fn test_ao() {
        let result = ao(1.5, 0.5);
        assert_eq!(result, 0.75);
    }
    #[test]
    fn test_anet() {
        let result = anet(17.5, 3.0, 3.5, 5.0, 0.75);
        assert_eq!(result, 85.25);
    }
    #[test]
    fn test_doverw() {
        let result = d_over_w(3.5, 5.0);
        assert_eq!(result, 1.42857142857143);
    }
}
