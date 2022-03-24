use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

use super::error::{PipelineError, Result};

pub struct GitTarget {
    pub url: String,
    pub commit: String,
}
impl GitTarget {
    pub fn repo(url: String) -> Self {
        Self {
            url,
            commit: String::from("HEAD"),
        }
    }
    pub fn on_commit(self, commit: String) -> Self {
        self.commit = commit;
        self
    }
}

pub trait Judge {
    fn judge(&self, target: GitTarget, from_path: PathBuf) -> Result<f64>;
}

#[derive(Debug, Deserialize)]
pub struct DockerJudge {
    image: String,
    copy_to: PathBuf,
    result_path: PathBuf,
}

impl Judge for DockerJudge {
    fn judge(&self, target: GitTarget, from_path: PathBuf) -> Result<f64> {
        // ./scripts/judge.sh {self.image} {repo_url} {self.path} {self.copy_to} {self.result_path}
        // TODO: use target commitHash
        let output = Command::new("./scripts/judge.sh")
            .arg(self.image)
            .arg(target.url)
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
