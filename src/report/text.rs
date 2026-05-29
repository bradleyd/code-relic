use crate::{
    Report, Result,
    report::{Renderer, penalty_label},
};

pub struct TextRenderer;

impl Renderer for TextRenderer {
    fn render(&self, report: &Report) -> Result<String> {
        let mut output = String::new();

        output.push_str("CodeRelic scan\n");
        output.push_str(&format!("Repo: {}\n", report.repo.name));
        output.push_str(&format!("Path: {}\n", report.repo.path));
        let languages = if report.languages.is_empty() {
            "none".to_string()
        } else {
            report
                .languages
                .iter()
                .map(|language| format!("{language:?}").to_lowercase())
                .collect::<Vec<_>>()
                .join(", ")
        };

        output.push_str(&format!("Detected languages: {languages}\n"));
        output.push('\n');
        output.push_str(&format!(
            "AI Change Readiness: {} / 100\n",
            report.scores.overall
        ));
        output.push_str(&format!(
            "Build Health:          {}\n",
            report.scores.build_health
        ));
        output.push_str(&format!(
            "Test Confidence:       {}\n",
            report.scores.test_confidence
        ));
        output.push_str(&format!(
            "Complexity Control:    {}\n",
            report.scores.complexity_control
        ));
        output.push_str(&format!(
            "Context Quality:       {}\n",
            report.scores.context_quality
        ));
        output.push_str(&format!(
            "Dependency Hygiene:    {}\n",
            report.scores.dependency_hygiene
        ));
        output.push_str(&format!(
            "Blast Radius Safety:   {}\n",
            report.scores.blast_radius_safety
        ));

        output.push('\n');
        output.push_str("Findings:\n");

        for finding in &report.findings {
            output.push_str(&format!(
                "- [{}] {}\n",
                penalty_label(finding.penalty),
                finding.title
            ));
            output.push_str(&format!("  {}\n", finding.description));
        }

        Ok(output)
    }
}
