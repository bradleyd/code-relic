use crate::{Category, Finding, Scores, Severity};

pub struct WeightedScorer;

impl WeightedScorer {
    pub fn new() -> Self {
        Self
    }

    pub fn score(&self, findings: &[Finding]) -> Scores {
        let mut build_health = RawScore::default();
        let mut test_confidence = RawScore::default();
        let mut complexity_control = RawScore::default();
        let mut context_quality = RawScore::default();
        let mut dependency_hygiene = RawScore::default();
        let mut blast_radius_safety = RawScore::default();

        for finding in findings {
            let penalty = penalty_for(finding.severity);

            match finding.category {
                Category::BuildHealth => build_health.subtract(penalty),
                Category::TestConfidence => test_confidence.subtract(penalty),
                Category::Complexity => complexity_control.subtract(penalty),
                Category::ContextQuality => context_quality.subtract(penalty),
                Category::DependencyHygiene => dependency_hygiene.subtract(penalty),
                Category::BlastRadius => blast_radius_safety.subtract(penalty),
            }
        }

        let build_health = build_health.as_u8();
        let test_confidence = test_confidence.as_u8();
        let complexity_control = complexity_control.as_u8();
        let context_quality = context_quality.as_u8();
        let dependency_hygiene = dependency_hygiene.as_u8();
        let blast_radius_safety = blast_radius_safety.as_u8();

        let overall = weighted_overall(
            build_health,
            test_confidence,
            complexity_control,
            context_quality,
            dependency_hygiene,
            blast_radius_safety,
        );

        Scores {
            overall,
            build_health,
            test_confidence,
            complexity_control,
            context_quality,
            dependency_hygiene,
            blast_radius_safety,
        }
    }
}

impl Default for WeightedScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
struct RawScore {
    value: i32,
}

impl Default for RawScore {
    fn default() -> Self {
        Self { value: 100 }
    }
}

impl RawScore {
    fn subtract(&mut self, penalty: i32) {
        self.value -= penalty;
    }

    fn as_u8(self) -> u8 {
        self.value.clamp(0, 100) as u8
    }
}

fn penalty_for(severity: Severity) -> i32 {
    match severity {
        Severity::Info => 0,
        Severity::Low => 4,
        Severity::Medium => 10,
        Severity::High => 25,
        Severity::Critical => 40,
    }
}

fn weighted_overall(
    build_health: u8,
    test_confidence: u8,
    complexity_control: u8,
    context_quality: u8,
    dependency_hygiene: u8,
    blast_radius_safety: u8,
) -> u8 {
    let overall = build_health as f32 * 0.25
        + test_confidence as f32 * 0.25
        + complexity_control as f32 * 0.20
        + context_quality as f32 * 0.10
        + dependency_hygiene as f32 * 0.10
        + blast_radius_safety as f32 * 0.10;

    overall.round() as u8
}
