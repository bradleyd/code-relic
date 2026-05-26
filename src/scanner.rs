use crate::{
    Config, Finding, Repo, Result,
    checks::{
        Check,
        common::{ci::CiCheck, large_files::LargeFilesCheck, readme::ReadmeCheck},
    },
};

pub struct Scanner {
    checks: Vec<Box<dyn Check>>,
}

impl Scanner {
    pub fn new(config: Config) -> Self {
        Self {
            checks: vec![
                Box::new(ReadmeCheck),
                Box::new(CiCheck),
                Box::new(LargeFilesCheck::new(config)),
            ],
        }
    }

    pub async fn scan(&self, repo: &Repo) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        for check in &self.checks {
            let mut check_findings = check.run(repo).await?;
            findings.append(&mut check_findings);
        }

        Ok(findings)
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
