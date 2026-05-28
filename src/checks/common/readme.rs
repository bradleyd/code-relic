use crate::{
    Category, Evidence, Finding, Repo, Result, Severity,
    checks::{Check, traits::CheckContext},
};

pub struct ReadmeCheck;

#[async_trait::async_trait]
impl Check for ReadmeCheck {
    fn id(&self) -> &'static str {
        "common.readme"
    }

    fn name(&self) -> &'static str {
        "README presence"
    }

    async fn run(&self, repo: &Repo, _ctx: &CheckContext) -> Result<Vec<Finding>> {
        let candidates = ["README.md", "README", "readme.md"];

        let found = candidates
            .iter()
            .any(|candidate| repo.path().join(candidate).exists());

        if found {
            Ok(vec![Finding {
                id: "common.readme.present".to_string(),
                title: "README found".to_string(),
                description: "A README file exists at the repository root.".to_string(),
                severity: Severity::Info,
                category: Category::ContextQuality,
                language: None,
                evidence: Evidence::Text {
                    detail: "README file found at repository root.".to_string(),
                },
            }])
        } else {
            Ok(vec![Finding {
                id: "common.readme.missing".to_string(),
                title: "README missing".to_string(),
                description: "No README file was found at the repository root. AI-assisted changes are riskier when basic project context is missing.".to_string(),
                severity: Severity::Medium,
                category: Category::ContextQuality,
                language: None,
                evidence: Evidence::Text {
                    detail: "Checked README.md, README, and readme.md.".to_string(),
                },
            }])
        }
    }
}
