use std::{path::PathBuf, process::Stdio, time::Duration};

use async_trait::async_trait;
use tokio::{process::Command, time};

use crate::Result;

#[derive(Debug, Clone)]
pub struct CommandSpec {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

#[async_trait]
pub trait CommandRunner: Send + Sync {
    async fn run(&self, spec: CommandSpec) -> Result<CommandOutput>;
}

#[derive(Debug, Default)]
pub struct TokioCommandRunner;

impl TokioCommandRunner {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandRunner for TokioCommandRunner {
    async fn run(&self, spec: CommandSpec) -> Result<CommandOutput> {
        let mut command = Command::new(&spec.program);

        command
            .args(&spec.args)
            .current_dir(&spec.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = command.spawn()?;

        let timeout = Duration::from_secs(spec.timeout_secs);

        match time::timeout(timeout, child.wait_with_output()).await {
            Ok(output) => {
                let output = output?;

                Ok(CommandOutput {
                    exit_code: output.status.code(),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    timed_out: false,
                })
            }
            Err(_) => Ok(CommandOutput {
                exit_code: None,
                stdout: String::new(),
                stderr: format!("command timed out after {} seconds", spec.timeout_secs),
                timed_out: true,
            }),
        }
    }
}
