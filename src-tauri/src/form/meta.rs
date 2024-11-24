use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug, PartialEq)]
pub struct Meta {
    pub quick_calc_compatible: bool,
}
