pub mod hrr_at_flashover;
pub mod maximum_enclosure_temperature;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Section8Method {
    HRRAtFlashover,
    MaximumEnclosureTemperature,
}

impl Section8Method {
    pub fn friendly_reference(&self) -> String {
        match self {
            Section8Method::HRRAtFlashover => {
                format!("Eq. 28, 29 & 33")
            }
            Section8Method::MaximumEnclosureTemperature => {
                format!("Eq. 42, 43 & 44")
            }
        }
    }
    pub fn method_description(&self) -> String {
        match self {
            Section8Method::HRRAtFlashover => "Equations 28, 29, and 33 in PD 7974-1 are key for calculating heat release rates in compartment fires. Equation 28, based on Thomas' analysis, estimates the heat release rate required for flashover, assuming a 600°C temperature rise in the upper layer. Equation 29, developed by McCaffrey et al., uses a 500°C temperature rise and includes heat transfer through enclosure boundaries. Equation 33, based on Kawagoe's work, calculates the heat release rate in a ventilation-controlled fire, where the fire is limited by oxygen supply. These equations are critical for fire behavior modeling and fire safety design.".to_string(),
            Section8Method::MaximumEnclosureTemperature => "Equations 41, 42, 43, and 44 in PD 7974-1 are used to estimate the maximum enclosure temperature after flashover. Equation 41, based on Thomas, Heselden, and Law's work, calculates the upper bound temperature Tg(max) as a function of ventilation parameters. Equation 42 defines the ventilation parameter (Ω), which depends on the enclosure's dimensions. If the fire load is low, Equations 43 and 44 adjust the maximum temperature estimate to account for insufficient energy. These equations are valid for specific enclosure sizes and thermal properties.".to_string(),
        }
    }
}
