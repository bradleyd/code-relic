use serde::{Deserialize, Serialize};

use super::{Category, Evidence, Language};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub language: Option<Language>,
    pub penalty: u8,
    pub evidence: Evidence,
}

impl Finding {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        category: Category,
        language: Option<Language>,
        penalty: u8,
        evidence: Evidence,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            category,
            language,
            penalty,
            evidence,
        }
    }
}
