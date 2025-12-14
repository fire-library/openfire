use pyo3::prelude::*;
use pyo3::wrap_pymodule;

// Import CIBSE Guide E chapter 6 appendix functions
use openfire::cibse_guide_e::chapter_6::appendix::{
    a_simple_case as rust_a_simple_case,
    b_more_than_one_window as rust_b_more_than_one_window,
    c_windows_in_more_than_one_wall as rust_c_windows_in_more_than_one_wall,
    common as rust_common,
    d_compartment_with_core as rust_d_compartment_with_core,
};

// A Simple Case module functions
#[pyfunction]
fn area_of_floor_a(w1: f64, w2: f64) -> PyResult<f64> {
    Ok(rust_a_simple_case::area_of_floor(w1, w2))
}

#[pyfunction]
fn area_of_opening_a(wo: f64, ho: f64) -> PyResult<f64> {
    Ok(rust_a_simple_case::area_of_opening(wo, ho))
}

#[pyfunction]
fn internal_surface_area_a(a_f: f64, h: f64, w1: f64, w2: f64, a_o: f64) -> PyResult<f64> {
    Ok(rust_a_simple_case::internal_surface_area(
        a_f, h, w1, w2, a_o,
    ))
}

#[pyfunction]
fn ratio_depth_over_width_a(w1: f64, w2: f64) -> PyResult<f64> {
    Ok(rust_a_simple_case::ratio_depth_over_width(w1, w2))
}

#[pymodule]
fn a_simple_case(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(area_of_floor_a, m)?)?;
    m.add_function(wrap_pyfunction!(area_of_opening_a, m)?)?;
    m.add_function(wrap_pyfunction!(internal_surface_area_a, m)?)?;
    m.add_function(wrap_pyfunction!(ratio_depth_over_width_a, m)?)?;
    Ok(())
}

// Common module functions
#[pyfunction]
fn area_of_floor_common(w1: f64, w2: f64) -> PyResult<f64> {
    Ok(rust_common::area_of_floor(w1, w2))
}

#[pyfunction]
fn area_of_opening_common(wo: f64, ho: f64) -> PyResult<f64> {
    Ok(rust_common::area_of_opening(wo, ho))
}

#[pyfunction]
fn internal_surface_area_common(a_f: f64, h: f64, w1: f64, w2: f64, a_o: f64) -> PyResult<f64> {
    Ok(rust_common::internal_surface_area(a_f, h, w1, w2, a_o))
}

#[pyfunction]
fn areas_of_openings_multiple_openings(openings_dimensions: Vec<(f64, f64)>) -> PyResult<Vec<f64>> {
    Ok(rust_common::areas_of_openings_multiple_openings(
        openings_dimensions,
    ))
}

#[pyfunction]
fn sum_areas_of_openings(areas_of_openings: Vec<f64>) -> PyResult<f64> {
    Ok(rust_common::sum_areas_of_openings(areas_of_openings))
}

#[pymodule]
fn common(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(area_of_floor_common, m)?)?;
    m.add_function(wrap_pyfunction!(area_of_opening_common, m)?)?;
    m.add_function(wrap_pyfunction!(internal_surface_area_common, m)?)?;
    m.add_function(wrap_pyfunction!(areas_of_openings_multiple_openings, m)?)?;
    m.add_function(wrap_pyfunction!(sum_areas_of_openings, m)?)?;
    Ok(())
}

// B More Than One Window module functions
#[pyfunction]
fn area_of_floor_b(w1: f64, w2: f64) -> PyResult<f64> {
    Ok(rust_b_more_than_one_window::area_of_floor(w1, w2))
}

#[pyfunction]
fn areas_of_openings_multiple_openings_b(
    openings_dimensions: Vec<(f64, f64)>,
) -> PyResult<Vec<f64>> {
    Ok(rust_b_more_than_one_window::areas_of_openings_multiple_openings(openings_dimensions))
}

#[pyfunction]
fn sum_areas_of_openings_b(areas_of_openings: Vec<f64>) -> PyResult<f64> {
    Ok(rust_b_more_than_one_window::sum_areas_of_openings(
        areas_of_openings,
    ))
}

#[pyfunction]
fn sum_width_of_compartment_openings_b(widths_of_openings: Vec<f64>) -> PyResult<f64> {
    Ok(rust_b_more_than_one_window::sum_width_of_compartment_openings(widths_of_openings))
}

#[pyfunction]
fn equivalent_height_for_compartment_openings_b(
    equivalent_area_of_openings: f64,
    equivalent_width_of_openings: f64,
) -> PyResult<f64> {
    Ok(
        rust_b_more_than_one_window::equivalent_height_for_compartment_openings(
            equivalent_area_of_openings,
            equivalent_width_of_openings,
        ),
    )
}

#[pymodule]
fn b_more_than_one_window(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(area_of_floor_b, m)?)?;
    m.add_function(wrap_pyfunction!(areas_of_openings_multiple_openings_b, m)?)?;
    m.add_function(wrap_pyfunction!(sum_areas_of_openings_b, m)?)?;
    m.add_function(wrap_pyfunction!(sum_width_of_compartment_openings_b, m)?)?;
    m.add_function(wrap_pyfunction!(
        equivalent_height_for_compartment_openings_b,
        m
    )?)?;
    Ok(())
}

// C Windows In More Than One Wall module functions
#[pyfunction]
fn area_of_floor_c(w1: f64, w2: f64) -> PyResult<f64> {
    Ok(rust_c_windows_in_more_than_one_wall::area_of_floor(w1, w2))
}

#[pyfunction]
fn sum_area_of_openings_per_wall(
    dimensions_of_openings_wall_per_wall: Vec<(f64, f64)>,
) -> PyResult<f64> {
    Ok(
        rust_c_windows_in_more_than_one_wall::sum_area_of_openings_per_wall(
            dimensions_of_openings_wall_per_wall,
        ),
    )
}

#[pyfunction]
fn sum_area_of_openigs(areas_of_openings: Vec<f64>) -> PyResult<f64> {
    Ok(rust_c_windows_in_more_than_one_wall::sum_area_of_openigs(
        areas_of_openings,
    ))
}

#[pyfunction]
fn ratio_depth_over_height(w1: f64, w2: f64, ao_w1: f64, ao: f64) -> PyResult<f64> {
    Ok(rust_c_windows_in_more_than_one_wall::ratio_depth_over_height(w1, w2, ao_w1, ao))
}

#[pymodule]
fn c_windows_in_more_than_one_wall(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(area_of_floor_c, m)?)?;
    m.add_function(wrap_pyfunction!(sum_area_of_openings_per_wall, m)?)?;
    m.add_function(wrap_pyfunction!(sum_area_of_openigs, m)?)?;
    m.add_function(wrap_pyfunction!(ratio_depth_over_height, m)?)?;
    Ok(())
}

// D Compartment With Core module functions
#[pyfunction]
fn floor_area_of_compartment_with_core(w1: f64, w2: f64, c1: f64, c2: f64) -> PyResult<f64> {
    Ok(rust_d_compartment_with_core::floor_area_of_compartment_with_core(w1, w2, c1, c2))
}

#[pyfunction]
fn internal_surface_area_of_compartment_with_core(
    w1: f64,
    w2: f64,
    h: f64,
    c1: f64,
    c2: f64,
    ch: f64,
    a_o: f64,
) -> PyResult<f64> {
    Ok(
        rust_d_compartment_with_core::internal_surface_area_of_compartment_with_core(
            w1, w2, h, c1, c2, ch, a_o,
        ),
    )
}

#[pyfunction]
fn ratio_depth_over_height_compartment_with_core(
    w1: f64,
    w2: f64,
    c1: f64,
    c2: f64,
    ao_w1: f64,
    ao: f64,
) -> PyResult<f64> {
    Ok(
        rust_d_compartment_with_core::ratio_depth_over_height_compartment_with_core(
            w1, w2, c1, c2, ao_w1, ao,
        ),
    )
}

#[pymodule]
fn d_compartment_with_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(floor_area_of_compartment_with_core, m)?)?;
    m.add_function(wrap_pyfunction!(
        internal_surface_area_of_compartment_with_core,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        ratio_depth_over_height_compartment_with_core,
        m
    )?)?;
    Ok(())
}

#[pymodule]
pub fn appendix(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(a_simple_case))?;
    m.add_wrapped(wrap_pymodule!(b_more_than_one_window))?;
    m.add_wrapped(wrap_pymodule!(c_windows_in_more_than_one_wall))?;
    m.add_wrapped(wrap_pymodule!(d_compartment_with_core))?;
    m.add_wrapped(wrap_pymodule!(common))?;
    Ok(())
}
