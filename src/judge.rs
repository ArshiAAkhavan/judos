use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

use crate::error::{PipelineError, Result};

pub type CommitHash = String;

pub trait Judge {
    fn judge(&self, repo_url: String, commit: CommitHash, from_path: PathBuf) -> Result<f64>;
}

#[derive(Debug, Deserialize)]
pub struct DockerJudge {
    image: String,
    copy_to: PathBuf,
    result_path: PathBuf,
}

impl Judge for DockerJudge {
    fn judge(&self, repo_url: String, commit: CommitHash, from_path: PathBuf) -> Result<f64> {
        // ./scripts/judge.sh {self.image} {repo_url} {self.path} {self.copy_to} {self.result_path}
        let output = Command::new("./scripts/judge.sh")
            .arg(self.image)
            .arg(repo_url)
            .arg(from_path)
            .arg(self.copy_to)
            .arg(self.result_path)
            .output()
            .expect("failed to execute judge script");
        match output.status.success() {
            true => String::from_utf8(output.stdout)
                .map(|s| s.parse::<f64>())
                .map_err(|_| PipelineError::MalformedOutput)?
                .map_err(|_| PipelineError::MalformedOutput),
            false => Err(PipelineError::TriggerError),
        }
    }
}
