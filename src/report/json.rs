use crate::{Report, Result, report::Renderer};

pub struct JsonRenderer;

impl Renderer for JsonRenderer {
    fn render(&self, report: &Report) -> Result<String> {
        let output = serde_json::to_string_pretty(report)?;
        Ok(output)
    }
}
