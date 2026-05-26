use serde::{Deserialize, Serialize};

use super::{Category, Evidence, Language, Severity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub category: Category,
    pub language: Option<Language>,
    pub evidence: Evidence,
}

impl Finding {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        severity: Severity,
        category: Category,
        language: Option<Language>,
        evidence: Evidence,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            severity,
            category,
            language,
            evidence,
        }
    }
}
