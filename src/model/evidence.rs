use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Evidence {
    Command {
        command: String,
        exit_code: Option<i32>,
        stdout_excerpt: String,
        stderr_excerpt: String,
    },
    Metric {
        name: String,
        value: f64,
        threshold: Option<f64>,
    },
    File {
        path: String,
        detail: String,
    },
    Text {
        detail: String,
    },
}
