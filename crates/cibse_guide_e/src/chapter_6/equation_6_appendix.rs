pub mod openfire_runner;

pub fn simplecase_af(w1: f64, w2: f64) -> f64 {
    return w1 * w2;
}

pub fn simplecase_ao(w_o: f64, h_o: f64) -> f64 {
    return w_o * h_o;
}

pub fn simplecase_anet(a_f: f64, h: f64, w1: f64, w2: f64, a_o: f64) -> f64 {
    return 2.0 * a_f + 2.0 * h * (w1 + w2) - a_o;
}

pub fn simplecase_dw(w1: f64, w2: f64) -> f64 {
    return w2 / w1;
}

pub fn equation_af(q_f: String, a_vo: String, h_o: String) -> String {
    format!("{} = 600 \\space {} \\sqrt {{{}}}", q_f, a_vo, h_o,)
}

pub fn equation_ao(q_f: String, a_vo: String, h_o: String) -> String {
    format!("{} = 600 \\space {} \\sqrt {{{}}}", q_f, a_vo, h_o,)
}

pub fn equation_anet(q_f: String, a_vo: String, h_o: String) -> String {
    format!("{} = 600 \\space {} \\sqrt {{{}}}", q_f, a_vo, h_o,)
}

pub fn equation_dw(q_f: String, a_vo: String, h_o: String) -> String {
    format!("{} = 600 \\space {} \\sqrt {{{}}}", q_f, a_vo, h_o,)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = simplecase_af(2.0, 2.1);
        assert_eq!(result, 1738.9652095427327);
    }
}
