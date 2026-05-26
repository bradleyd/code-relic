#[derive(Debug, Clone)]
pub struct Config {
    pub excludes: Vec<String>,
    pub large_file_warning_lines: usize,
    pub large_file_high_lines: usize,
    pub command_timeout_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            excludes: vec![
                ".git".to_string(),
                "target".to_string(),
                "node_modules".to_string(),
                "vendor".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
            large_file_warning_lines: 500,
            large_file_high_lines: 1000,
            command_timeout_secs: 120,
        }
    }
}
