pub enum Tag {
    SprinklerActivation,
    FireScenario,
    HRR,
    FireDynamics,
    Ventilation,
}

impl Tag {
    pub fn to_string(&self) -> String {
        match self {
            Tag::SprinklerActivation => "Sprinkler Activation".to_string(),
            Tag::FireScenario => "Fire Scenario".to_string(),
            Tag::HRR => "Heat Release Rate".to_string(),
            Tag::FireDynamics => "Fire Dynamics".to_string(),
            Tag::Ventilation => "Ventilation".to_string(),
        }
    }
}
