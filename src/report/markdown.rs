use crate::{Language, Report, Result, report::Renderer};

pub struct MarkdownRenderer;

impl Renderer for MarkdownRenderer {
    fn render(&self, report: &Report) -> Result<String> {
        let mut output = String::new();

        output.push_str("# CodeRelic Report\n\n");

        output.push_str("## Repository\n\n");
        output.push_str(&format!("- **Name:** {}\n", report.repo.name));
        output.push_str(&format!("- **Path:** `{}`\n", report.repo.path));
        output.push_str(&format!(
            "- **Rust project:** {}\n",
            if report.languages.contains(&Language::Rust) {
                "yes"
            } else {
                "no"
            }
        ));
        output.push_str(&format!("- **CodeRelic version:** `{}`\n", report.version));

        output.push_str("\n## AI Change Readiness\n\n");
        output.push_str(&format!(
            "**Overall:** `{}` / 100\n\n",
            report.scores.overall
        ));

        output.push_str("| Category | Score |\n");
        output.push_str("|---|---:|\n");
        output.push_str(&format!(
            "| Build Health | {} |\n",
            report.scores.build_health
        ));
        output.push_str(&format!(
            "| Test Confidence | {} |\n",
            report.scores.test_confidence
        ));
        output.push_str(&format!(
            "| Complexity Control | {} |\n",
            report.scores.complexity_control
        ));
        output.push_str(&format!(
            "| Context Quality | {} |\n",
            report.scores.context_quality
        ));
        output.push_str(&format!(
            "| Dependency Hygiene | {} |\n",
            report.scores.dependency_hygiene
        ));
        output.push_str(&format!(
            "| Blast Radius Safety | {} |\n",
            report.scores.blast_radius_safety
        ));

        output.push_str("\n## Findings\n\n");

        for finding in &report.findings {
            output.push_str(&format!(
                "### {:?}: {}\n\n",
                finding.severity, finding.title
            ));
            output.push_str(&format!("{}\n\n", finding.description));
            output.push_str(&format!("- **ID:** `{}`\n", finding.id));
            output.push_str(&format!("- **Category:** `{:?}`\n", finding.category));

            if let Some(language) = finding.language {
                output.push_str(&format!("- **Language:** `{:?}`\n", language));
            }

            output.push('\n');
        }

        Ok(output)
    }
}
