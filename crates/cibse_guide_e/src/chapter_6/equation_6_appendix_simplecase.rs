pub mod openfire_runner;

pub fn af(w1: f64, w2: f64) -> f64 {
    return w1 * w2;
}

pub fn ao(w_o: f64, h_o: f64) -> f64 {
    return w_o * h_o;
}

pub fn anet(a_f: f64, h: f64, w1: f64, w2: f64, a_o: f64) -> f64 {
    return 2.0 * a_f + 2.0 * h * (w1 + w2) - a_o;
}

pub fn d_over_w(w1: f64, w2: f64) -> f64 {
    return w2 / w1;
}

pub fn equation_af(a_f: String, w1: String, w2: String) -> String {
    format!("{} = {} \\space {}", a_f, w1, w2)
}

pub fn equation_ao(a_o: String, w_o: String, h_o: String) -> String {
    format!("{} = {} \\space {}", a_o, w_o, h_o)
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

pub fn equation_doverw(d_over_w: String, w1: String, w2: String) -> String {
    format!("{} = {} / {}", d_over_w, w2, w1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_af() {
        let result = af(4.0, 5.0);
        assert_eq!(result, 20.000);
    }

    #[test]
    fn test_ao() {
        let result = ao(2.0, 1.5);
        assert_eq!(result, 3.000);
    }
    #[test]
    fn test_anet() {
        let result = anet(20.0, 3.0, 4.0, 5.0, 3.0);
        assert_eq!(result, 91.000);
    }
    #[test]
    fn test_doverw() {
        let result = d_over_w(4.0, 5.0);
        assert_eq!(result, 1.25);
    }
}
