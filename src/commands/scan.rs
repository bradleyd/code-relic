use std::{fs, path::PathBuf};

use clap::{Parser, ValueEnum};

use crate::{
    Config, JsonRenderer, MarkdownRenderer, Renderer, Repo, Report, Result, TextRenderer,
    WeightedScorer, scanner::Scanner,
};

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Debug, Parser)]
pub struct ScanArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long, value_enum, default_value = "text")]
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

    let languages = scanner.detected_languages(&repo);

    let report = Report {
        tool: "coderelic".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        repo: repo.summary(),
        languages,
        scores,
        findings,
    };

    let rendered = match args.format {
        OutputFormat::Text => TextRenderer.render(&report)?,
        OutputFormat::Json => JsonRenderer.render(&report)?,
        OutputFormat::Markdown => MarkdownRenderer.render(&report)?,
    };

    if let Some(output_path) = args.output {
        fs::write(output_path, rendered)?;
    } else {
        println!("{rendered}");
    }

    if let Some(minimum_score) = args.fail_under
        && report.scores.overall < minimum_score
    {
        std::process::exit(1);
    }
    Ok(())
}
