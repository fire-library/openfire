pub enum Tag {
    Evacuation,
    ExternalFireSpread,
    FireDynamics,
    FireScenario,
    HRR,
    Radiation,
    SprinklerActivation,
    Ventilation,
    ViewFactor,
}

impl Tag {
    pub fn to_string(&self) -> String {
        match self {
            Tag::Evacuation => "Evacuation".to_string(),
            Tag::ExternalFireSpread => "External Fire Spread".to_string(),
            Tag::FireDynamics => "Fire Dynamics".to_string(),
            Tag::FireScenario => "Fire Scenario".to_string(),
            Tag::HRR => "Heat Release Rate".to_string(),
            Tag::Radiation => "Radiation".to_string(),
            Tag::SprinklerActivation => "Sprinkler Activation".to_string(),
            Tag::Ventilation => "Ventilation".to_string(),
            Tag::ViewFactor => "View Factor".to_string(),
        }
    }
}
