pub use super::common;

pub fn area_of_floor(w1: f64, w2: f64) -> f64 {
    return common::area_of_floor(w1, w2);
}

pub fn area_of_floor_equation(a_f: String, w1: String, w2: String) -> String {
    return common::area_of_floor_equation(a_f, w1, w2);
}

pub fn area_of_opening(wo: f64, ho: f64) -> f64 {
    return common::area_of_opening(wo, ho);
}

pub fn area_of_opening_equation(a_o: String, w_o: String, h_o: String) -> String {
    return common::area_of_opening_equation(a_o, w_o, h_o);
}

pub fn internal_surface_area(a_f: f64, h: f64, w1: f64, w2: f64, a_o: f64) -> f64 {
    return common::internal_surface_area(a_f, h, w1, w2, a_o);
}

pub fn internal_surface_area_equation(
    a_net: String,
    a_f: String,
    h: String,
    w1: String,
    w2: String,
    a_o: String,
) -> String {
    return common::internal_surface_area_equation(a_net, a_f, h, w1, w2, a_o);
}

pub fn ratio_depth_over_height(w1: f64, w2: f64) -> f64 {
    return common::ratio_depth_over_width(w1, w2);
}

pub fn ratio_depth_over_height_equation(d_over_w: String, w1: String, w2: String) -> String {
    return common::ratio_depth_over_width_equation(d_over_w, w1, w2);
}
