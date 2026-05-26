use serde::{Deserialize, Serialize};

use super::{Finding, Language, Scores};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub tool: String,
    pub version: String,
    pub repo: RepoSummary,
    pub languages: Vec<Language>,
    pub scores: Scores,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSummary {
    pub path: String,
    pub name: String,
}
