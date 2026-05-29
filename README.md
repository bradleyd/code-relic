# CodeRelic

CodeRelic is a Rust CLI that scans a codebase and estimates how ready it is for AI-assisted maintenance.

It does not try to judge your architecture, recommend rewrites, or tell you to move frameworks. Instead, it checks practical signals that affect whether AI-generated code changes are likely to be safe, reviewable, and easy to validate.

## What CodeRelic checks today

CodeRelic currently focuses on Rust projects and basic repository health.

It checks:

- README presence
- CI configuration presence
- Large source files
- cargo check --all-targets
- cargo test --no-run
- cargo clippy --all-targets --all-features
- cargo fmt --check
- cargo metadata --format-version 1 --no-deps

The result is an evidence-backed readiness score with category-level subscores.

## Example

```text
CodeRelic scan Repo: code_relic
Path: /Users/example/Projects/code_relic
Detected languages: rust
AI Change Readiness: 95 / 100
Build Health:          85
Test Confidence:       100
Complexity Control:    100
Context Quality:       85
Dependency Hygiene:    100
Blast Radius Safety:   100
Findings: 
- [Moderate] README missing   No README file was found at the repository root. AI-assisted changes are riskier when basic project context is missing.
- [Moderate] CI configuration missing   No common CI configuration was found. AI-assisted changes are riskier when validation only happens manually or locally.
- [Info] cargo check passed   The Rust project passes cargo check. This improves confidence that the baseline code can be validated before and after AI-assisted changes.
```

## Installation

CodeRelic is currently early-stage. For now, run it from source:

```bash
cargo run -- scan . 
```

Or build it locally:

```bash
cargo build --release ./target/release/coderelic scan . 
```

## Usage

Scan the current directory:

```bash
coderelic scan .
```

Scan another repository:

```bash
coderelic scan /path/to/repo
```

Output JSON:

```bash
coderelic scan . --format json
```

Output Markdown:

```bash
coderelic scan . --format markdown
```

Write a report to a file:

```bash
coderelic scan . --format markdown --output coderelic-report.md
```

Fail if the score is below a threshold:

```bash
coderelic scan . --fail-under 70
```

## Score categories

CodeRelic reports one overall score and several category scores.

### AI Change Readiness

The overall estimate of how safe and practical the codebase appears for AI-assisted maintenance.

### Build Health

Whether the project has a working validation baseline, such as passing cargo check and having CI configuration.

### Test Confidence

Whether test targets compile and there is evidence that behavior can be validated.

### Complexity Control

Whether the codebase appears reviewable and manageable based on early signals such as file size, formatting, and linting.

### Context Quality

Whether the repository provides enough project context for humans and AI tools to work safely.

### Dependency Hygiene

Whether CodeRelic can inspect the project’s dependency and package shape.

### Blast Radius Safety

An estimate of how contained future changes are likely to be. This category is intentionally light in the current version and will become more useful as CodeRelic gains deeper project-shape analysis.

## Design philosophy

CodeRelic follows a few rules:

1. Findings should be evidence-backed.
2. Scores should come from deterministic checks, not an LLM.
3. Passing checks should provide confidence, not penalties.
4. Failed or missing checks should explain why they matter for AI-assisted change.
5. The tool should avoid architecture advice and rewrite recommendations.

The goal is not to shame a codebase. The goal is to understand the risk before handing it to an AI coding agent.

## Current status

CodeRelic is in early development.

The first phase is focused on Rust projects and deterministic local checks. Future work may include:

- Python support
- Node/TypeScript support
- Better dependency analysis
- Safer blast-radius estimates
- Report comparison
- AI editing guidance files
- Optional local LLM explanations

## License

TBD
