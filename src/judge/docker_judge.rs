use std::{
    path::{Path, PathBuf},
    process::Command,
};

use log::{error, info, warn};
use serde::{Deserialize, Serialize};

use super::GitTarget;
use super::Judge;

use crate::error::{JudosError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerJudge {
    image: String,
    copy_to: PathBuf,
    result_path: PathBuf,
}

const JUDGE_SCRIPT_FILE_PATH: &str = "./scripts/judge.sh";

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
        info!("{}", String::from_utf8_lossy(&output.stdout));
        match output.status.success() {
            true => String::from_utf8(output.stdout)
                .map(|s| s.trim().parse::<f64>())
                .map_err(|_| JudosError::MalformedOutput)?
                .map_err(|_| JudosError::MalformedOutput),
            false => {
                warn!("running judge failed on ({target},{from_path:?})");
                error!("{}", String::from_utf8(output.stderr).unwrap());
                Err(JudosError::TriggerError)
            }
        }
    }
}
