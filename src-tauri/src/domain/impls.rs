pub mod br187;
pub mod introduction_to_fire_dynamics;
pub mod pd7974;
pub mod sfpe_handbook;

use crate::domain::method::MethodType;

pub enum Icon {
    FireIcon,
}

pub struct Implementation {
    pub name: String,
    pub tags: Vec<String>,
    pub description: String,
    pub reference: String,
    pub method_type: MethodType,
    pub icon: Icon,
    pub colors: String,
}

pub fn all_impls() -> Vec<Implementation> {
    vec![]
}
