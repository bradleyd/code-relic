use crate::{Finding, Repo, Result};

#[async_trait::async_trait]
pub trait Check: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;

    async fn run(&self, repo: &Repo) -> Result<Vec<Finding>>;
}
