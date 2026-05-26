use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::{Config, Language, Repo, Report, Result, WeightedScorer, scanner::Scanner};

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Terminal,
    Json,
    Markdown,
}

#[derive(Debug, Parser)]
pub struct ScanArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long, value_enum, default_value = "terminal")]
    pub format: OutputFormat,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[arg(long)]
    pub fail_under: Option<u8>,
}

pub async fn run(args: ScanArgs) -> Result<()> {
    let repo = Repo::new(&args.path)?;

    let scanner = Scanner::new(Config::default());
    let findings = scanner.scan(&repo).await?;
    let scorer = WeightedScorer::new();
    let scores = scorer.score(&findings);

    let languages = if repo.is_rust() {
        vec![Language::Rust]
    } else {
        Vec::new()
    };

    let report = Report {
        tool: "coderelic".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        repo: repo.summary(),
        languages,
        scores,
        findings,
    };

    println!("CodeRelic scan");
    println!("Repo: {}", report.repo.name);
    println!("Path: {}", report.repo.path);
    println!(
        "Rust project: {}",
        if report.languages.contains(&Language::Rust) {
            "yes"
        } else {
            "no"
        }
    );

    println!();
    println!("AI Change Readiness: {} / 100", report.scores.overall);
    println!("Build Health:          {}", report.scores.build_health);
    println!("Test Confidence:       {}", report.scores.test_confidence);
    println!(
        "Complexity Control:    {}",
        report.scores.complexity_control
    );
    println!("Context Quality:       {}", report.scores.context_quality);
    println!(
        "Dependency Hygiene:    {}",
        report.scores.dependency_hygiene
    );
    println!(
        "Blast Radius Safety:   {}",
        report.scores.blast_radius_safety
    );

    for finding in &report.findings {
        println!("- [{:?}] {}", finding.severity, finding.title);
        println!("  {}", finding.description);
    }

    let _format = args.format;
    let _output = args.output;
    let _fail_under = args.fail_under;

    Ok(())
}
