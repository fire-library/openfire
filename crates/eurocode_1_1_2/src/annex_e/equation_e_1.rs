pub fn design_fire_load(q_f_k: f64, m: f64, delta_q_1: f64, delta_q_2: f64, delta_n: f64) -> f64 {
    q_f_k * m * delta_q_1 * delta_q_2 * delta_n
}

pub fn design_fire_load_equation(
    q_f_d: String,
    q_f_k: String,
    m: String,
    delta_q_1: String,
    delta_q_2: String,
    delta_n: String,
) -> String {
    format!(
        "{} = {} * {} * {} * {} * {}",
        q_f_d, q_f_k, m, delta_q_1, delta_q_2, delta_n
    )
}

pub fn delta_n(
    delta_n_1: f64,
    delta_n_2: f64,
    delta_n_3: f64,
    delta_n_4: f64,
    delta_n_5: f64,
    delta_n_6q: f64,
    delta_n_7: f64,
    delta_n_8: f64,
    delta_n_9: f64,
    delta_n_10: f64,
) -> f64 {
    delta_n_1
        * delta_n_2
        * delta_n_3
        * delta_n_4
        * delta_n_5
        * delta_n_6q
        * delta_n_7
        * delta_n_8
        * delta_n_9
        * delta_n_10
}

pub fn delta_n_equation(
    delta_n: String,
    delta_n_1: String,
    delta_n_2: String,
    delta_n_3: String,
    delta_n_4: String,
    delta_n_5: String,
    delta_n_6: String,
    delta_n_7: String,
    delta_n_8: String,
    delta_n_9: String,
    delta_n_10: String,
) -> String {
    format!(
        "{} = {} * {} * {} * {} * {} * {} * {} * {} * {} * {}",
        delta_n,
        delta_n_1,
        delta_n_2,
        delta_n_3,
        delta_n_4,
        delta_n_5,
        delta_n_6,
        delta_n_7,
        delta_n_8,
        delta_n_9,
        delta_n_10
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_fire_load() {
        let delta_n_variable = delta_n(0.61,
            1.0,
            0.87, 
            0.87,
            0.87,
            0.61,
            0.61,
            0.9,
            1.0,
            1.0);
        let result = design_fire_load(780.0, 0.8, 1.1, 0.78, delta_n_variable);
        let expected = 72.02141503058;
        assert!((result - expected).abs() < 1e-6);
    }
}
