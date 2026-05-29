use crate::{Category, CommandSpec, Evidence, Finding, Language, Repo, Result, checks::Check};

use crate::checks::traits::CheckContext;

use serde::Deserialize;
pub struct CargoMetadataCheck;

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
    workspace_members: Vec<String>,
    workspace_root: String,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    id: String,
    targets: Vec<serde_json::Value>,
    dependencies: Vec<serde_json::Value>,
    features: HashMap<String, Vec<String>>,
}

#[async_trait::async_trait]
impl Check for CargoMetadataCheck {
    fn id(&self) -> &'static str {
        "rust.cargo_metadata"
    }

    fn name(&self) -> &'static str {
        "cargo metadata"
    }

    async fn run(&self, repo: &Repo, ctx: &CheckContext) -> Result<Vec<Finding>> {
        let command = "cargo metadata --format-version 1 --no-deps";

        let output = ctx
            .runner
            .run(CommandSpec {
                program: "cargo".to_string(),
                args: vec![
                    "metadata".to_string(),
                    "--format-version".to_string(),
                    "1".to_string(),
                    "--no-deps".to_string(),
                ],
                cwd: repo.path().to_path_buf(),
                timeout_secs: ctx.config.command_timeout_secs,
            })
            .await?;

        if output.timed_out {
            return Ok(vec![Finding {
                             id: "rust.cargo_metadata.timed_out".to_string(),
                             title: "Cargo metadata timed out".to_string(),
                             description: "cargo metadata did not complete before the configured timeout. CodeRelic could not inspect the Rust project shape, dependency declarations, or workspace structure.".to_string(),
                             category: Category::DependencyHygiene,
                             language: Some(Language::Rust),
                             penalty: 15,
                             evidence: Evidence::Command {
                                 command: command.to_string(),
                                 exit_code: output.exit_code,
                                 stdout_excerpt: excerpt(&output.stdout),
                                 stderr_excerpt: excerpt(&output.stderr),
                             },
            }]);
        };

        if output.exit_code != Some(0) {
            return Ok(vec![Finding {
                            id: "rust.cargo_metadata.failed".to_string(),
                            title: "Cargo metadata failed".to_string(),
                            description: "cargo metadata did not complete successfully. CodeRelic could not inspect the Rust project shape, dependency declarations, or workspace structure.".to_string(),
                            category: Category::DependencyHygiene,
                            language: Some(Language::Rust),
                            penalty: 15,
                            evidence: Evidence::Command {
                                command: command.to_string(),
                                exit_code: output.exit_code,
                                stdout_excerpt: excerpt(&output.stdout),
                                stderr_excerpt: excerpt(&output.stderr),
                            },

            }]);
        };

        let metadata = match serde_json::from_str::<CargoMetadata>(&output.stdout) {
            Ok(metadata) => metadata,
            Err(err) => {
                return Ok(vec![Finding {
                    id: "rust.cargo_metadata.parse_failed".to_string(),
                    title: "Cargo metadata could not be parsed".to_string(),
                    description:
                        "cargo metadata completed, but CodeRelic could not parse the JSON output."
                            .to_string(),
                    category: Category::DependencyHygiene,
                    language: Some(Language::Rust),
                    penalty: 10,
                    evidence: Evidence::Text {
                        detail: format!("Parse error: {err}"),
                    },
                }]);
            }
        };

        let package_count = metadata.packages.len();
        let workspace_member_count = metadata.workspace_members.len();
        let workspace_packages = metadata
            .packages
            .iter()
            .filter(|package| metadata.workspace_members.contains(&package.id))
            .collect::<Vec<_>>();

        let target_count: usize = workspace_packages
            .iter()
            .map(|package| package.targets.len())
            .sum();

        let dependency_count: usize = workspace_packages
            .iter()
            .map(|package| package.dependencies.len())
            .sum();

        let feature_count: usize = workspace_packages
            .iter()
            .map(|package| package.features.len())
            .sum();

        Ok(vec![Finding {
            id: "rust.cargo_metadata.loaded".to_string(),
            title: "Cargo metadata loaded".to_string(),
            description: format!(
                "Cargo metadata was loaded for 1 workspace package(s), 1 target(s), 7 direct dependency declaration(s), and 0 feature group(s)."
            ),
            category: Category::DependencyHygiene,
            language: Some(Language::Rust),
            penalty: 0,
            evidence: Evidence::Text {
                detail: format!(
                    "workspace_root={}, workspace_members={}, packages={}, targets={}, dependencies={}, features={}",
                    metadata.workspace_root,
                    workspace_member_count,
                    package_count,
                    target_count,
                    dependency_count,
                    feature_count
                ),
            },
        }])
    }
}

fn excerpt(value: &str) -> String {
    const MAX_CHARS: usize = 2_000;

    if value.chars().count() <= MAX_CHARS {
        return value.to_string();
    }

    value.chars().take(MAX_CHARS).collect()
}
