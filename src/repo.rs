use std::path::{Path, PathBuf};

use crate::{Error, RepoSummary, Result};

#[derive(Debug, Clone)]
pub struct Repo {
    path: PathBuf,
}

impl Repo {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() || !path.is_dir() {
            return Err(Error::InvalidRepo(path.to_path_buf()));
        }

        let path = path.canonicalize()?;

        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> String {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    pub fn is_rust(&self) -> bool {
        self.path.join("Cargo.toml").exists()
    }

    pub fn summary(&self) -> RepoSummary {
        RepoSummary {
            path: self.path.display().to_string(),
            name: self.name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_directory_can_be_a_repo() {
        let repo = Repo::new(".");
        assert!(repo.is_ok());
    }
}
