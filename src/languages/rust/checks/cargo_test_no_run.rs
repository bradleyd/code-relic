use crate::{Category, CommandSpec, Evidence, Finding, Language, Repo, Result, checks::Check};

use crate::checks::traits::CheckContext;

pub struct CargoTestNoRun;

#[async_trait::async_trait]
impl Check for CargoTestNoRun {
    fn id(&self) -> &'static str {
        "rust.cargo_test_no_run_check"
    }

    fn name(&self) -> &'static str {
        "cargo test no-run check"
    }

    async fn run(&self, repo: &Repo, ctx: &CheckContext) -> Result<Vec<Finding>> {
        let command = "cargo test --no-run";

        let output = ctx
            .runner
            .run(CommandSpec {
                program: "cargo".to_string(),
                args: vec!["test".to_string(), "--no-run".to_string()],
                cwd: repo.path().to_path_buf(),
                timeout_secs: ctx.config.command_timeout_secs,
            })
            .await?;

        let finding = if output.timed_out {
            Finding {
                id: "rust.cargo_test_no_run.timed_out".to_string(),
                title: "cargo test no run timed out".to_string(),
                description: "cargo test --no-run did not complete before the configured timeout. AI-assisted changes are riskier when test feedback is slow or unavailable.".to_string(),
                category: Category::BuildHealth,
                language: Some(Language::Rust),
                penalty: 30,
                evidence: Evidence::Command {
                    command: command.to_string(),
                    exit_code: output.exit_code,
                    stdout_excerpt: excerpt(&output.stdout),
                    stderr_excerpt: excerpt(&output.stderr),
                },
            }
        } else if output.exit_code == Some(0) {
            Finding {
                id: "rust.cargo_test_no_run.passed".to_string(),
                title: "cargo test no-run passed".to_string(),
                description: "The Rust project’s test targets compile. This improves confidence that AI-assisted changes can be validated before tests are executed.".to_string(),
                category: Category::BuildHealth,
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
                id: "rust.cargo_test_no_run.failed".to_string(),
                title: "cargo check failed".to_string(),
                description: "The Rust project’s test targets do not compile. AI-assisted behavior changes are riskier when the test harness is already broken.".to_string(),
                category: Category::BuildHealth,
                language: Some(Language::Rust),
                penalty: 30,
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
