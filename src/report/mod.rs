pub mod json;
pub mod markdown;
pub mod text;

pub use json::JsonRenderer;
pub use markdown::MarkdownRenderer;
pub use text::TextRenderer;

use crate::{Report, Result};

pub trait Renderer {
    fn render(&self, report: &Report) -> Result<String>;
}
