use framework::method::runner::{MethodRunner, Reference};
use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct DocumentImplementations {
    pub document: String,
    pub document_id: String,
    pub implementations: Vec<framework::method::implementation::Implementation>,
}

pub fn all_impls() -> Vec<DocumentImplementations> {
    vec![
        DocumentImplementations {
            document: br_187::BR187::Document.document_name(),
            document_id: br_187::BR187::Document.document_id(),
            implementations: vec![
                br_187::chapter_1::equation_1::openfire_runner::BR187Chapter1Equation1Builder.index_page(),
                br_187::appendix_a::openfire_runner::BR187AppendixAViewFactorsBuilder.index_page(),
            ],
        },
        DocumentImplementations {
            document: bs9999::BS9999::Document.document_name(),
            document_id: bs9999::BS9999::Document.document_id(),
            implementations: vec![
                bs9999::chapter_15::figure_6a::openfire_runner::BS9999Chapter15Figure6aBuilder.index_page(),
                bs9999::chapter_15::figure_6b::openfire_runner::BS9999Chapter15Figure6bBuilder.index_page(),
                bs9999::chapter_15::figure_6c::openfire_runner::BS9999Chapter15Figure6cBuilder.index_page(),
            ],
        },
        DocumentImplementations {
            document: cibse_guide_e::CIBSEGuideE::Document.document_name(),
            document_id: cibse_guide_e::CIBSEGuideE::Document.document_id(),
            implementations: vec![
                cibse_guide_e::chapter_6::equation_6_7::openfire_runner::Chapter6Equation7Runner.index_page(),
                cibse_guide_e::chapter_6::equation_6_55::openfire_runner::Chapter6Equation55Runner.index_page(),
                cibse_guide_e::chapter_6::equation_6_57::openfire_runner::Chapter6Equation57Runner.index_page(),
                cibse_guide_e::chapter_6::equation_6_58::openfire_runner::Chapter6Equation58Runner.index_page(),
                cibse_guide_e::chapter_7::equation_7_2::openfire_runner::Chapter7Equation2Runner.index_page(),
                cibse_guide_e::chapter_7::equation_7_3::openfire_runner::Chapter7Equation3Runner.index_page(),
                cibse_guide_e::chapter_7::equation_7_6::openfire_runner::Chapter7Equation6Runner.index_page(),
                cibse_guide_e::chapter_10::equation_10_1::openfire_runner::Chapter10Equation1Runner.index_page(),
                cibse_guide_e::chapter_10::equation_10_2::openfire_runner::Chapter10Equation2Runner.index_page(),
                cibse_guide_e::chapter_10::equation_10_3::openfire_runner::Chapter10Equation3Runner.index_page(),
                cibse_guide_e::chapter_10::equation_10_4::openfire_runner::Chapter10Equation4Runner.index_page(),
                cibse_guide_e::chapter_10::equation_10_7::openfire_runner::Chapter10Equation7Runner.index_page(),
                cibse_guide_e::chapter_10::equation_10_8::openfire_runner::Chapter10Equation8Runner.index_page(),
                cibse_guide_e::chapter_10::equation_10_10::openfire_runner::Chapter10Equation10Runner.index_page(),
                cibse_guide_e::chapter_10::equation_10_11::openfire_runner::Chapter10Equation11Runner.index_page(),
                cibse_guide_e::chapter_10::equation_10_12::openfire_runner::Chapter10Equation12Runner.index_page(),
            ]
        },
        DocumentImplementations {
            document: introduction_to_fire_dynamics::IntroductionToFireDynamics::Document.document_name(),
            document_id: introduction_to_fire_dynamics::IntroductionToFireDynamics::Document.document_id(),
            implementations: vec![
        introduction_to_fire_dynamics::chapter_10::equation_10_18::openfire_runner::BurningRegimeBuilder.index_page()
            ],
        },
        DocumentImplementations {
            document: pd_7974::PD7974::One(pd_7974::part_1::Section::Document).document_name(),
            document_id: pd_7974::PD7974::One(pd_7974::part_1::Section::Document).document_id(),
            implementations: vec![
        pd_7974::part_1::section_8::maximum_enclosure_temperature_runner::MaximumEnclosureTemperatureBuilder.index_page(),
        pd_7974::part_1::section_8::hrr_at_flashover_runner::HRRAtFlashoverBuilder.index_page(),
        pd_7974::part_1::section_8::maximum_hrr_runner::MaximumHRRBuilder.index_page(),
            ],
        },
        DocumentImplementations {
            document: sfpe_handbook::SFPEHandbook::Document.document_name(),
            document_id: sfpe_handbook::SFPEHandbook::Document.document_id(),
            implementations: vec![
        sfpe_handbook::chapter_14::alpert::openfire_runner::AlpertHeatReleaseFromTempAndPositionBuilder.index_page(),
            ],
        },
        DocumentImplementations {
            document: tr17::TR17::Document.document_name(),
            document_id: tr17::TR17::Document.document_id(),
            implementations: vec![
                tr17::section_2::equation_1::openfire_runner::TR17Section2Equation1Builder.index_page(),
            ]
        },
    ]
}

pub fn register_runners() {
    framework::register_runner::<
        br_187::chapter_1::equation_1::openfire_runner::BR187Chapter1Equation1Builder,
    >();
    framework::register_runner::<
        br_187::appendix_a::openfire_runner::BR187AppendixAViewFactorsBuilder,
    >();
    framework::register_runner::<
        bs9999::chapter_15::figure_6a::openfire_runner::BS9999Chapter15Figure6aBuilder,
    >();
    framework::register_runner::<
        bs9999::chapter_15::figure_6b::openfire_runner::BS9999Chapter15Figure6bBuilder,
    >();
    framework::register_runner::<
        bs9999::chapter_15::figure_6c::openfire_runner::BS9999Chapter15Figure6cBuilder,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_6::equation_6_7::openfire_runner::Chapter6Equation7Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_6::equation_6_55::openfire_runner::Chapter6Equation55Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_6::equation_6_57::openfire_runner::Chapter6Equation57Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_6::equation_6_58::openfire_runner::Chapter6Equation58Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_7::equation_7_2::openfire_runner::Chapter7Equation2Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_7::equation_7_3::openfire_runner::Chapter7Equation3Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_7::equation_7_6::openfire_runner::Chapter7Equation6Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_10::equation_10_1::openfire_runner::Chapter10Equation1Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_10::equation_10_2::openfire_runner::Chapter10Equation2Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_10::equation_10_3::openfire_runner::Chapter10Equation3Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_10::equation_10_4::openfire_runner::Chapter10Equation4Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_10::equation_10_7::openfire_runner::Chapter10Equation7Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_10::equation_10_8::openfire_runner::Chapter10Equation8Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_10::equation_10_10::openfire_runner::Chapter10Equation10Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_10::equation_10_11::openfire_runner::Chapter10Equation11Runner,
    >();
    framework::register_runner::<
        cibse_guide_e::chapter_10::equation_10_12::openfire_runner::Chapter10Equation12Runner,
    >();
    framework::register_runner::<
        introduction_to_fire_dynamics::chapter_10::equation_10_18::openfire_runner::BurningRegimeBuilder,
    >();
    framework::register_runner::<pd_7974::part_1::section_8::maximum_enclosure_temperature_runner::MaximumEnclosureTemperatureBuilder>();
    framework::register_runner::<
        pd_7974::part_1::section_8::hrr_at_flashover_runner::HRRAtFlashoverBuilder,
    >();
    framework::register_runner::<pd_7974::part_1::section_8::maximum_hrr_runner::MaximumHRRBuilder>(
    );
    framework::register_runner::<sfpe_handbook::chapter_14::alpert::openfire_runner::AlpertHeatReleaseFromTempAndPositionBuilder>();
    framework::register_runner::<
        tr17::section_2::equation_1::openfire_runner::TR17Section2Equation1Builder,
    >();
}
