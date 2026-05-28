use std::sync::Arc;

use crate::{
    Config, Finding, Language, Repo, Result, TokioCommandRunner,
    checks::{
        Check,
        common::{ci::CiCheck, large_files::LargeFilesCheck, readme::ReadmeCheck},
        traits::CheckContext,
    },
    languages::{LanguageAnalyzer, rust::RustAnalyzer},
};

pub struct Scanner {
    common_checks: Vec<Box<dyn Check>>,
    analyzers: Vec<Box<dyn LanguageAnalyzer>>,
    ctx: CheckContext,
}

impl Scanner {
    pub fn new(config: Config) -> Self {
        let ctx = CheckContext {
            config,

            runner: Arc::new(TokioCommandRunner::new()),
        };

        Self {
            common_checks: vec![
                Box::new(ReadmeCheck),
                Box::new(CiCheck),
                Box::new(LargeFilesCheck),
            ],
            analyzers: vec![Box::new(RustAnalyzer::new())],
            ctx,
        }
    }

    pub async fn scan(&self, repo: &Repo) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        for check in &self.common_checks {
            let mut check_findings = check.run(repo, &self.ctx).await?;
            findings.append(&mut check_findings);
        }

        for analyzer in &self.analyzers {
            if analyzer.detect(repo) {
                let mut analyzer_findings = analyzer.analyze(repo, &self.ctx).await?;

                findings.append(&mut analyzer_findings);
            }
        }

        Ok(findings)
    }

    pub fn detected_languages(&self, repo: &Repo) -> Vec<Language> {
        self.analyzers
            .iter()
            .filter(|analyzer| analyzer.detect(repo))
            .map(|analyzer| analyzer.language())
            .collect()
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
