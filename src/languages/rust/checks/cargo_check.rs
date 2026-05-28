use crate::{
    Category, CommandSpec, Evidence, Finding, Language, Repo, Result, Severity, checks::Check,
};

use crate::checks::traits::CheckContext;

pub struct CargoCheck;

#[async_trait::async_trait]
impl Check for CargoCheck {
    fn id(&self) -> &'static str {
        "rust.cargo_check"
    }

    fn name(&self) -> &'static str {
        "cargo check"
    }

    async fn run(&self, repo: &Repo, ctx: &CheckContext) -> Result<Vec<Finding>> {
        let command = "cargo check --all-targets";

        let output = ctx
            .runner
            .run(CommandSpec {
                program: "cargo".to_string(),
                args: vec!["check".to_string(), "--all-targets".to_string()],
                cwd: repo.path().to_path_buf(),
                timeout_secs: ctx.config.command_timeout_secs,
            })
            .await?;

        let finding = if output.timed_out {
            Finding {
                id: "rust.cargo_check.timed_out".to_string(),
                title: "cargo check timed out".to_string(),
                description: "cargo check did not complete before the configured timeout. AI-assisted changes are riskier when the build feedback loop is slow or unavailable.".to_string(),
                severity: Severity::High,
                category: Category::BuildHealth,
                language: Some(Language::Rust),
                evidence: Evidence::Command {
                    command: command.to_string(),
                    exit_code: output.exit_code,
                    stdout_excerpt: excerpt(&output.stdout),
                    stderr_excerpt: excerpt(&output.stderr),
                },
            }
        } else if output.exit_code == Some(0) {
            Finding {
                id: "rust.cargo_check.passed".to_string(),
                title: "cargo check passed".to_string(),
                description: "The Rust project passes cargo check. This improves confidence that the baseline code can be validated before and after AI-assisted changes.".to_string(),
                severity: Severity::Info,
                category: Category::BuildHealth,
                language: Some(Language::Rust),
                evidence: Evidence::Command {
                    command: command.to_string(),
                    exit_code: output.exit_code,
                    stdout_excerpt: excerpt(&output.stdout),
                    stderr_excerpt: excerpt(&output.stderr),
                },
            }
        } else {
            Finding {
                id: "rust.cargo_check.failed".to_string(),
                title: "cargo check failed".to_string(),
                description: "The Rust project does not currently pass cargo check. AI-assisted changes are riskier when the baseline build is already failing.".to_string(),
                severity: Severity::High,
                category: Category::BuildHealth,
                language: Some(Language::Rust),
                evidence: Evidence::Command {
                    command: command.to_string(),
                    exit_code: output.exit_code,
                    stdout_excerpt: excerpt(&output.stdout),
                    stderr_excerpt: excerpt(&output.stderr),
                },
            }
        };

        Ok(vec![finding])
    }
}

fn excerpt(value: &str) -> String {
    const MAX_CHARS: usize = 2_000;

    if value.chars().count() <= MAX_CHARS {
        return value.to_string();
    }

    value.chars().take(MAX_CHARS).collect()
}
