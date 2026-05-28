use std::sync::Arc;

use crate::{CommandRunner, Config, Finding, Repo, Result};

pub struct CheckContext {
    pub config: Config,
    pub runner: Arc<dyn CommandRunner>,
}

#[async_trait::async_trait]
pub trait Check: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;

    async fn run(&self, repo: &Repo, ctx: &CheckContext) -> Result<Vec<Finding>>;
}
