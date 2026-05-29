use crate::{
    Category, Evidence, Finding, Repo, Result,
    checks::{Check, traits::CheckContext},
};

pub struct CiCheck;

#[async_trait::async_trait]
impl Check for CiCheck {
    fn id(&self) -> &'static str {
        "common.ci"
    }

    fn name(&self) -> &'static str {
        "CI presence"
    }

    async fn run(&self, repo: &Repo, _ctx: &CheckContext) -> Result<Vec<Finding>> {
        let candidates = [
            ".github/workflows",
            ".gitlab-ci.yml",
            ".circleci/config.yml",
            "buildkite",
        ];

        let found = candidates
            .iter()
            .find(|candidate| repo.path().join(candidate).exists());

        let finding = match found {
            Some(path) => Finding {
                id: "common.ci.present".to_string(),
                title: "CI configuration found".to_string(),
                description: "A CI configuration was found. This improves confidence that changes can be validated outside a developer machine.".to_string(),
                category: Category::BuildHealth,
                language: None,
                penalty: 0,
                evidence: Evidence::File {
                    path: path.to_string(),
                    detail: "CI configuration exists.".to_string(),
                },
            },
            None => Finding {
                id: "common.ci.missing".to_string(),
                title: "CI configuration missing".to_string(),
                description: "No common CI configuration was found. AI-assisted changes are riskier when validation only happens manually or locally.".to_string(),
                category: Category::BuildHealth,
                language: None,
                penalty: 15,
                evidence: Evidence::Text {
                    detail: "Checked .github/workflows, .gitlab-ci.yml, .circleci/config.yml, and buildkite.".to_string(),
                },
            },
        };

        Ok(vec![finding])
    }
}
