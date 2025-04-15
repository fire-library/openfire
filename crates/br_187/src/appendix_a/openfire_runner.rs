mod surface_radiation;
mod total_radiation;

use framework::method::calculation::Calculation;
use framework::method::equation::Dependency;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ParameterValue;
use framework::method::parameter::Parameters;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::parameter::object::Object;
use framework::method::parameter::{ArcParameter, ParameterTrait};
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::validation::ParameterError;
use framework::method::{Method, step::Step};
use std::sync::{Arc, RwLock};
use std::vec;

use crate::BR187;
use crate::appendix_a::Method as MethodRef;

#[derive(Default)]
pub struct BR187AppendixAViewFactorsBuilder;

struct Symbols {
    surfaces: &'static str,
    boltzman: &'static str,
    total_radiation: &'static str,
}

struct SurfaceSymbols {
    name: &'static str,
    h: &'static str,
    emissivity: &'static str,
    temperature: &'static str,
    w: &'static str,
    s: &'static str,
    alignment: &'static str,
    additive: &'static str,
    phi: &'static str,
    x: &'static str,
    y: &'static str,
    i_s: &'static str,
    i_r: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    surfaces: "Radiating Surfaces",
    boltzman: "\\sigma",
    total_radiation: "Q",
};

const SURFACE_SYMBOLS: SurfaceSymbols = SurfaceSymbols {
    name: "\\text{Name}",
    h: "H",
    emissivity: "\\epsilon",
    temperature: "T",
    w: "W",
    s: "S",
    alignment: "\\text{Alignment}",
    additive: "\\text{Add or Subtract View Factor?}",
    phi: "\\phi",
    x: "X",
    y: "Y",
    i_s: "I_s",
    i_r: "I_r",
};

const PAR_CENTER_ALIGNED: &str = "Parallel Center Aligned";
const PAR_EDGE_ALIGNED: &str = "Parallel Corner Aligned";
const PER_EDGE_ALIGNED: &str = "Perpendicular Corner Aligned";

const ADDITIVE: &str = "Add";
const SUBTRACTIVE: &str = "Subtract";

impl BR187AppendixAViewFactorsBuilder {
    pub fn example_surface() -> ArcParameter {
        let name = ParamBuilder::string(SURFACE_SYMBOLS.name)
            .name("Name")
            .required()
            .default_value(Some(ParameterValue::String(
                "Radiating Surface".to_string(),
            )))
            .build();

        let h = ParamBuilder::float(&SURFACE_SYMBOLS.h)
            .name("Height")
            .units("m")
            .required()
            .min_exclusive(0.0)
            .build();

        let emissivity = ParamBuilder::float(&SURFACE_SYMBOLS.emissivity)
            .name("emissivity")
            .required()
            .min(0.0)
            .max(1.0)
            .default_value(Some(ParameterValue::Float(1.0)))
            .build();

        let temperature = ParamBuilder::float(&SURFACE_SYMBOLS.temperature)
            .name("Temperature of Surface")
            .units("K")
            .required()
            .min(0.0)
            .build();

        let w = ParamBuilder::float(&SURFACE_SYMBOLS.w)
            .name("Width")
            .units("m")
            .required()
            .min_exclusive(0.0)
            .build();

        let s = ParamBuilder::float(&SURFACE_SYMBOLS.s)
            .name("Distance to Receiving Surface")
            .units("m")
            .required()
            .min_exclusive(0.0)
            .build();

        let alignment = ParamBuilder::string_enum(
            &SURFACE_SYMBOLS.alignment,
            vec![PAR_CENTER_ALIGNED, PAR_EDGE_ALIGNED, PER_EDGE_ALIGNED],
        )
        .name("Alignment")
        .required()
        .default_value(Some(ParameterValue::String(PAR_CENTER_ALIGNED.to_string())))
        .build();

        let additive =
            ParamBuilder::string_enum(&SURFACE_SYMBOLS.additive, vec![ADDITIVE, SUBTRACTIVE])
                .name("Add or Subtract View Factor?")
                .required()
                .default_value(Some(ParameterValue::String(ADDITIVE.to_string())))
                .build();

        let x = ParamBuilder::output_float(&SURFACE_SYMBOLS.x)
            .name("X")
            .build();

        let y = ParamBuilder::output_float(&SURFACE_SYMBOLS.y)
            .name("Y")
            .build();

        let phi = ParamBuilder::output_float(&SURFACE_SYMBOLS.phi)
            .name("View Factor")
            .build();

        let i_s = ParamBuilder::output_float(&SURFACE_SYMBOLS.i_s)
            .name("Intensity of radiation")
            .units("\\frac{kW}{m^{2}}")
            .build();

        let i_r = ParamBuilder::output_float(&SURFACE_SYMBOLS.i_r)
            .name("Intensity of radiation on receiving surface")
            .units("\\frac{kW}{m^{2}}")
            .build();

        let mut params = Parameters::new();
        params.add(name);
        params.add(temperature);
        params.add(emissivity);
        params.add(h);
        params.add(w);
        params.add(s);
        params.add(x);
        params.add(y);
        params.add(alignment);
        params.add(additive);
        params.add(phi);
        params.add(i_s);
        params.add(i_r);

        ParamBuilder::object(&SYMBOLS.surfaces, params)
            .name("Radiating Surface")
            .build()
    }
}

impl MethodRunner for BR187AppendixAViewFactorsBuilder {
    fn name(&self) -> String {
        "Transfer of Heat by Thermal Radiation".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &BR187::AppendixA(MethodRef::ViewFactors)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::ViewFactor, Tag::Radiation, Tag::ExternalFireSpread]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the Incident Heat Flux on a receiving surface".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let total_radiation = params.get(SYMBOLS.total_radiation);
        Some(vec![total_radiation])
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let surfaces = ParamBuilder::list(SYMBOLS.surfaces)
            .name("Radiating Surfaces")
            .default_value(Some(ParameterValue::List(vec![Self::example_surface()])))
            .build();

        let boltzman = ParamBuilder::float(SYMBOLS.boltzman)
            .name("Stefan Boltzmann constant")
            .units("\\frac{kW}{m^{2}K^{4}}")
            .default_value(Some(ParameterValue::Float(5.67e-11)))
            .build();

        let total_radiation = ParamBuilder::output_float(SYMBOLS.total_radiation)
            .name("Total Radiation at Receiver")
            .units("\\frac{kW}{m^{2}}")
            .build();

        params.add(boltzman);
        params.add(surfaces);
        params.add(total_radiation);

        return params;
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let mut fields = vec![];

        let surfaces = params.get(SYMBOLS.surfaces).as_list();
        let surface_names = surfaces
            .into_iter()
            .map(|s| s.as_object().values())
            .flatten()
            .filter(|s| s.name() == SURFACE_SYMBOLS.name)
            .collect::<Vec<ArcParameter>>();

        for param in surface_names.clone() {
            param.unique_parameters(surface_names.clone());
        }

        fields.push(
            params
                .get(SYMBOLS.surfaces)
                .to_field_list(Self::example_surface),
        );

        let step_1 = FormStep {
            name: "Ventilation Factor Input".to_string(),
            description: "Input required to calculate the ventilation factor".to_string(),
            fields: fields,
            introduction: vec![],
        };

        Form::new(vec![step_1])
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let total_radiation = params.get(&SYMBOLS.total_radiation);
        let surfaces = params.get(&SYMBOLS.surfaces).as_list();

        let boltzman = params.get(&SYMBOLS.boltzman);
        let Object(object) = surfaces[0].as_object();
        let h = object.get(&SURFACE_SYMBOLS.h);
        let w = object.get(&SURFACE_SYMBOLS.w);
        let s = object.get(&SURFACE_SYMBOLS.s);
        let emissivity = object.get(&SURFACE_SYMBOLS.emissivity);
        let temp = object.get(&SURFACE_SYMBOLS.temperature);
        let phi = object.get(&SURFACE_SYMBOLS.phi);
        let i_s = object.get(&SURFACE_SYMBOLS.i_s);
        let i_r = object.get(&SURFACE_SYMBOLS.i_r);

        let input = surfaces
            .iter()
            .map(|s| {
                let Object(object) = s.as_object();
                vec![
                    object.get(&SURFACE_SYMBOLS.h),
                    object.get(&SURFACE_SYMBOLS.w),
                    object.get(&SURFACE_SYMBOLS.s),
                    object.get(&SURFACE_SYMBOLS.emissivity),
                    object.get(&SURFACE_SYMBOLS.temperature),
                ]
                .into_iter()
                .map(|d| Dependency {
                    parameter: d,
                    identifier: Some(object.get(&SURFACE_SYMBOLS.name).as_string()),
                })
                .collect::<Vec<Dependency>>()
            })
            .flatten()
            .collect::<Vec<Dependency>>();

        let step_1 = Step {
            name: "View Factors".to_string(),
            nomenclature: vec![h, w, s, emissivity, boltzman, temp, phi, i_s, i_r.clone()],
            input: input,
            process: surface_radiation::process(),
            calculation: surface_radiation::calculation(surfaces.clone()),
            render: true,
        };

        let input = surfaces
            .iter()
            .map(|s| {
                let Object(object) = s.as_object();
                let rad = object.get(&SURFACE_SYMBOLS.i_r);

                Dependency {
                    parameter: rad,
                    identifier: Some(object.get(&SURFACE_SYMBOLS.name).as_string()),
                }
            })
            .collect::<Vec<Dependency>>();

        let step_2 = Step {
            name: "Total Radiation at Receiver".to_string(),
            nomenclature: vec![i_r.clone(), total_radiation.clone()],
            input: input,
            process: total_radiation::process(),
            calculation: total_radiation::calculation(total_radiation, surfaces),
            render: true,
        };

        let write_sheet = calc_sheet.clone();
        let mut calc = write_sheet.write().unwrap();
        calc.add_step(step_1);
        calc.add_step(step_2);

        calc_sheet
    }

    fn evaluate(&self, method: &mut Method) -> Result<(), Vec<ParameterError>> {
        let boltzman = method.parameters.get(&SYMBOLS.boltzman).as_float();
        let surfaces = method.parameters.get(&SYMBOLS.surfaces).as_list();

        for surface in surfaces.clone() {
            let Object(object) = surface.as_object();
            let h = object.get(&SURFACE_SYMBOLS.h).as_float();
            let w = object.get(&SURFACE_SYMBOLS.w).as_float();
            let s = object.get(&SURFACE_SYMBOLS.s).as_float();
            let x = object.get(&SURFACE_SYMBOLS.x);
            let y = object.get(&SURFACE_SYMBOLS.y);
            let alignment = object.get(&SURFACE_SYMBOLS.alignment).as_string();
            let emissivity = object.get(&SURFACE_SYMBOLS.emissivity).as_float();
            let temp = object.get(&SURFACE_SYMBOLS.temperature).as_float();
            let phi = object.get(&SURFACE_SYMBOLS.phi);
            let i_s = object.get(&SURFACE_SYMBOLS.i_s);
            let i_r = object.get(&SURFACE_SYMBOLS.i_r);
            let additive = if object.get(&SURFACE_SYMBOLS.additive).as_string() == ADDITIVE {
                true
            } else {
                false
            };

            if alignment == PAR_CENTER_ALIGNED {
                let x_result = crate::appendix_a::equation_a3::x(w, s);
                let y_result = crate::appendix_a::equation_a3::y(h, s);
                let view_factor = crate::appendix_a::equation_a3::phi(x_result, y_result, additive);

                let radiation_emitted =
                    crate::appendix_a::equation_a1::radiation_intensity(boltzman, emissivity, temp);
                let radiation_received =
                    crate::appendix_a::equation_a2::radiation_intensity_at_receiver(
                        view_factor,
                        radiation_emitted,
                    );

                x.update(Some(x_result.to_string()))?;
                y.update(Some(y_result.to_string()))?;
                phi.update(Some(view_factor.to_string()))?;
                i_s.update(Some(radiation_emitted.to_string()))?;
                i_r.update(Some(radiation_received.to_string()))?;
            } else if alignment == PAR_EDGE_ALIGNED {
                let x_result = crate::appendix_a::equation_a4::x(w, s);
                let y_result = crate::appendix_a::equation_a4::y(h, s);
                let view_factor = crate::appendix_a::equation_a4::phi(x_result, y_result, additive);

                let radiation_emitted =
                    crate::appendix_a::equation_a1::radiation_intensity(boltzman, emissivity, temp);
                let radiation_received =
                    crate::appendix_a::equation_a2::radiation_intensity_at_receiver(
                        view_factor,
                        radiation_emitted,
                    );

                x.update(Some(x_result.to_string()))?;
                y.update(Some(y_result.to_string()))?;
                phi.update(Some(view_factor.to_string()))?;
                i_s.update(Some(radiation_emitted.to_string()))?;
                i_r.update(Some(radiation_received.to_string()))?;
            } else if alignment == PER_EDGE_ALIGNED {
                let x_result = crate::appendix_a::equation_a5::x(w, s);
                let y_result = crate::appendix_a::equation_a5::y(h, s);
                let view_factor = crate::appendix_a::equation_a5::phi(x_result, y_result, additive);

                let radiation_emitted =
                    crate::appendix_a::equation_a1::radiation_intensity(boltzman, emissivity, temp);
                let radiation_received =
                    crate::appendix_a::equation_a2::radiation_intensity_at_receiver(
                        view_factor,
                        radiation_emitted,
                    );

                x.update(Some(x_result.to_string()))?;
                y.update(Some(y_result.to_string()))?;
                phi.update(Some(view_factor.to_string()))?;
                i_s.update(Some(radiation_emitted.to_string()))?;
                i_r.update(Some(radiation_received.to_string()))?;
            }
        }

        let receiver_radiation: f64 = surfaces
            .into_iter()
            .map(|s| s.as_object().0.get(&SURFACE_SYMBOLS.i_r).as_float())
            .sum();

        let total_radiation = method.parameters.get(&SYMBOLS.total_radiation);
        total_radiation.update(Some(receiver_radiation.to_string()))?;

        return Ok(());
    }
}
