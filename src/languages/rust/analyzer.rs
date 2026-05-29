use crate::{
    Finding, Language, Repo, Result,
    checks::{Check, traits::CheckContext},
    languages::LanguageAnalyzer,
    languages::rust::checks::CargoCheck,
    languages::rust::checks::CargoTestNoRun,
};

pub struct RustAnalyzer {
    checks: Vec<Box<dyn Check>>,
}

impl RustAnalyzer {
    pub fn new() -> Self {
        Self {
            checks: vec![Box::new(CargoCheck), Box::new(CargoTestNoRun)],
        }
    }
}

impl Default for RustAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl LanguageAnalyzer for RustAnalyzer {
    fn language(&self) -> Language {
        Language::Rust
    }

    fn detect(&self, repo: &Repo) -> bool {
        // this needs to look up one directory and down one directory too.
        repo.path().join("Cargo.toml").exists()
    }

    async fn analyze(&self, repo: &Repo, ctx: &CheckContext) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        for check in &self.checks {
            let mut check_findings = check.run(repo, ctx).await?;
            findings.append(&mut check_findings);
        }

        Ok(findings)
    }
}
