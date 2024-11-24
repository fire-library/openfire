use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Type, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParametricFire {
    id: String,
    area_of_vertical_openings: f64,
    // fire_load: FireLoadInput,
    // materials: Vec<MaterialInput>,
    timestep: f64,
    // walls: Vec<WallInput>,
}
