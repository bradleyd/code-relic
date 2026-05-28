use crate::{Finding, Language, Repo, Result, checks::traits::CheckContext};

#[async_trait::async_trait]
pub trait LanguageAnalyzer: Send + Sync {
    fn language(&self) -> Language;

    fn detect(&self, repo: &Repo) -> bool;

    async fn analyze(&self, repo: &Repo, ctx: &CheckContext) -> Result<Vec<Finding>>;
}
