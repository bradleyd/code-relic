use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::{
    Repo, Result,
    checks::{Check, common::readme::ReadmeCheck},
};

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

    let check = ReadmeCheck;
    let findings = check.run(&repo).await?;

    println!("CodeRelic scan");
    println!("Repo: {}", repo.name());
    println!("Path: {}", repo.path().display());
    println!(
        "Rust project: {}",
        if repo.is_rust() { "yes" } else { "no" }
    );

    println!();
    println!("Findings:");

    for finding in findings {
        println!("- [{:?}] {}", finding.severity, finding.title);
        println!("  {}", finding.description);
    }

    let _format = args.format;
    let _output = args.output;
    let _fail_under = args.fail_under;

    Ok(())
}
