pub mod br187;
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
    vec![Implementation {
        name: "BR 187 Basic".to_string(),
        tags: vec!["BR 187".to_string(), "Basic".to_string()],
        description: "BR 187 Basic calculation".to_string(),
        reference: "BR 187".to_string(),
        method_type: MethodType::PD7974Part2Section7Equation1,
        icon: Icon::FireIcon,
        colors: "red".to_string(),
    }]
}
