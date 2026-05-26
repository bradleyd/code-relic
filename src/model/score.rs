use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scores {
    pub overall: u8,
    pub build_health: u8,
    pub test_confidence: u8,
    pub complexity_control: u8,
    pub context_quality: u8,
    pub dependency_hygiene: u8,
    pub blast_radius_safety: u8,
}
