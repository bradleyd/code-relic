use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use walkdir::WalkDir;

use crate::{
    Category, Config, Evidence, Finding, Repo, Result, Severity,
    checks::{Check, traits::CheckContext},
};

pub struct LargeFilesCheck;

impl LargeFilesCheck {
    fn should_skip(&self, path: &Path, config: &Config) -> bool {
        path.components().any(|component| {
            let component = component.as_os_str().to_string_lossy();

            config
                .excludes
                .iter()
                .any(|excluded| component == excluded.as_str())
        })
    }

    fn is_source_like(path: &Path) -> bool {
        matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("rs")
                | Some("py")
                | Some("js")
                | Some("ts")
                | Some("tsx")
                | Some("jsx")
                | Some("go")
                | Some("java")
                | Some("rb")
                | Some("ex")
                | Some("exs")
                | Some("erl")
                | Some("c")
                | Some("h")
                | Some("cpp")
                | Some("hpp")
        )
    }

    fn count_lines(path: &Path) -> Result<usize> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(reader.lines().count())
    }
}

#[async_trait::async_trait]
impl Check for LargeFilesCheck {
    fn id(&self) -> &'static str {
        "common.large_files"
    }

    fn name(&self) -> &'static str {
        "Large source files"
    }

    async fn run(&self, repo: &Repo, ctx: &CheckContext) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        for entry in WalkDir::new(repo.path())
            .into_iter()
            .filter_entry(|entry| !self.should_skip(entry.path(), &ctx.config))
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let path = entry.path();

            if !Self::is_source_like(path) {
                continue;
            }

            let line_count = Self::count_lines(path)?;

            if line_count < ctx.config.large_file_warning_lines {
                continue;
            }

            let severity = if line_count >= ctx.config.large_file_high_lines {
                Severity::High
            } else {
                Severity::Medium
            };

            let relative_path = path
                .strip_prefix(repo.path())
                .unwrap_or(path)
                .display()
                .to_string();

            findings.push(Finding {
                id: "common.large_file.detected".to_string(),
                title: "Large source file detected".to_string(),
                description: format!(
                    "{relative_path} has {line_count} lines. Large files can increase AI-change risk because more behavior is concentrated in one place."
                ),
                severity,
                category: Category::Complexity,
                language: None,
                evidence: Evidence::Metric {
                    name: "line_count".to_string(),
                    value: line_count as f64,
                    threshold: Some(ctx.config.large_file_warning_lines as f64),
                },
            });
        }

        if findings.is_empty() {
            findings.push(Finding {
                id: "common.large_files.none".to_string(),
                title: "No large source files detected".to_string(),
                description: "No source-like files exceeded the configured large-file threshold."
                    .to_string(),
                severity: Severity::Info,
                category: Category::Complexity,
                language: None,
                evidence: Evidence::Text {
                    detail: format!(
                        "No source-like files exceeded {} lines.",
                        ctx.config.large_file_warning_lines
                    ),
                },
            });
        }

        Ok(findings)
    }
}
