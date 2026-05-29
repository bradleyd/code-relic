use crate::{Category, CommandSpec, Evidence, Finding, Language, Repo, Result, checks::Check};

use crate::checks::traits::CheckContext;

pub struct CargoFmt;

#[async_trait::async_trait]
impl Check for CargoFmt {
    fn id(&self) -> &'static str {
        "rust.cargo_fmt"
    }

    fn name(&self) -> &'static str {
        "cargo fmt check"
    }

    async fn run(&self, repo: &Repo, ctx: &CheckContext) -> Result<Vec<Finding>> {
        let command = "cargo fmt --check";

        let output = ctx
            .runner
            .run(CommandSpec {
                program: "cargo".to_string(),
                args: vec!["fmt".to_string(), "--check".to_string()],
                cwd: repo.path().to_path_buf(),
                timeout_secs: ctx.config.command_timeout_secs,
            })
            .await?;

        let finding = if output.timed_out {
            Finding {
                id: "rust.cargo_fmt.timed_out".to_string(),
                title: "cargo fmt timed out".to_string(),
                description: "cargo fmt --check did not complete before the configured timeout. AI-assisted changes are riskier when formatting is not consistent.".to_string(),
                category: Category::Complexity,
                language: Some(Language::Rust),
                penalty: 5,
                evidence: Evidence::Command {
                    command: command.to_string(),
                    exit_code: output.exit_code,
                    stdout_excerpt: excerpt(&output.stdout),
                    stderr_excerpt: excerpt(&output.stderr),
                },
            }
        } else if output.exit_code == Some(0) {
            Finding {
                id: "rust.cargo_fmt.passed".to_string(),
                title: "cargo fmt passed".to_string(),
                description: "cargo fmt --check passed. The Rust project has a consistent formatting baseline, which reduces noisy diffs during AI-assisted changes.".to_string(),
                category: Category::Complexity,
                language: Some(Language::Rust),
                penalty: 0,
                evidence: Evidence::Command {
                    command: command.to_string(),
                    exit_code: output.exit_code,
                    stdout_excerpt: excerpt(&output.stdout),
                    stderr_excerpt: excerpt(&output.stderr),
                },
            }
        } else {
            Finding {
                id: "rust.cargo_fmt.failed".to_string(),
                title: "cargo fmt failed".to_string(),
                description: "cargo fmt --check did not pass. This declines confidence that AI-assisted changes can be validated.".to_string(),
                category: Category::Complexity,
                language: Some(Language::Rust),
                penalty: 5,
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
