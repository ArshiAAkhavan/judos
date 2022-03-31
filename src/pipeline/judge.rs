use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};
use std::process::Command;

use log::{error, warn};
use serde::{Deserialize, Serialize};

use super::error::{PipelineError, Result};

const JUDGE_SCRIPT_FILE_PATH: &str = "./scripts/judge.sh";

#[derive(Debug)]
pub struct GitTarget {
    pub url: String,
    pub commit: String,
}
impl Display for GitTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GitTarget({}::{})", self.get_name(), self.commit)
    }
}
impl GitTarget {
    pub fn repo(url: String) -> Self {
        Self {
            url,
            commit: String::from("HEAD"),
        }
    }
    pub fn on_commit(mut self, commit: String) -> Self {
        self.commit = commit;
        self
    }

    pub fn get_name(&self) -> &str {
        let name = self.url.split('/').last().unwrap();
        name.strip_suffix(".git").unwrap_or(name)
    }
}

#[typetag::serde]
pub trait Judge: Send + Sync + Debug {
    fn judge(&self, target: &GitTarget, from_path: &Path) -> Result<f64>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerJudge {
    image: String,
    copy_to: PathBuf,
    result_path: PathBuf,
}

#[typetag::serde(name = "docker")]
impl Judge for DockerJudge {
    fn judge(&self, target: &GitTarget, from_path: &Path) -> Result<f64> {
        // ./scripts/judge.sh {self.image} {repo_url} {self.path} {self.copy_to} {self.result_path}
        let output = Command::new(JUDGE_SCRIPT_FILE_PATH)
            .arg(&self.image)
            .arg(&target.url)
            .arg(from_path)
            .arg(&self.copy_to)
            .arg(&self.result_path)
            .arg(&target.commit)
            .output()
            .expect("failed to execute judge script");
        match output.status.success() {
            true => String::from_utf8(output.stdout)
                .map(|s| s.trim().parse::<f64>())
                .map_err(|_| PipelineError::MalformedOutput)?
                .map_err(|_| PipelineError::MalformedOutput),
            false => {
                warn!("running judge failed on ({target},{from_path:?})");
                error!("{}", String::from_utf8(output.stderr).unwrap());
                Err(PipelineError::TriggerError)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DummyJudge;

#[typetag::serde(name = "dummy")]
impl Judge for DummyJudge {
    fn judge(&self, _target: &GitTarget, _from_path: &Path) -> Result<f64> {
        Ok(0.0f64)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConstJudge {
    score: f64,
}

#[typetag::serde(name = "const")]
impl Judge for ConstJudge {
    fn judge(&self, _target: &GitTarget, _from_path: &Path) -> Result<f64> {
        Ok(self.score)
    }
}
