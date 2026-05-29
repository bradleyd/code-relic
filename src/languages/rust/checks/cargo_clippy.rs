use crate::{
    Category, CommandSpec, Evidence, Finding, Language, Repo, Result, Severity, checks::Check,
};

use crate::checks::traits::CheckContext;

pub struct CargoClippy;

#[async_trait::async_trait]
impl Check for CargoClippy {
    fn id(&self) -> &'static str {
        "rust.cargo_clippy"
    }

    fn name(&self) -> &'static str {
        "cargo clippy check"
    }

    async fn run(&self, repo: &Repo, ctx: &CheckContext) -> Result<Vec<Finding>> {
        let command = "cargo clippy --all-targets --all-features";

        let output = ctx
            .runner
            .run(CommandSpec {
                program: "cargo".to_string(),
                args: vec![
                    "clippy".to_string(),
                    "--all-targets".to_string(),
                    "--all-features".to_string(),
                ],
                cwd: repo.path().to_path_buf(),
                timeout_secs: ctx.config.command_timeout_secs,
            })
            .await?;

        let finding = if output.timed_out {
            Finding {
                id: "rust.cargo_clippy.timed_out".to_string(),
                title: "cargo clippy timed out".to_string(),
                description: "cargo clippy did not complete before the configured timeout. AI-assisted changes are riskier when test feedback is slow or unavailable.".to_string(),
                severity: Severity::Medium,
                category: Category::Complexity,
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
                id: "rust.cargo_clippy.passed".to_string(),
                title: "cargo clippy passed".to_string(),
                description: "cargo clippy --all-targets --all-features passed. This improves confidence that AI-assisted changes can be validated.".to_string(),
                severity: Severity::Info,
                category: Category::Complexity,
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
                id: "rust.cargo_clippy.failed".to_string(),
                title: "cargo clippy failed".to_string(),
                description: "cargo clippy --all-targets --all-features did not pass. This may indicate lint issues, feature-combination problems, or project-specific build assumptions.".to_string(),
                severity: Severity::Medium,
                category: Category::Complexity,
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
