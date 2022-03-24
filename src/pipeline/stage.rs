use std::{path::PathBuf, process::Command, time};

use super::error::Result;
use super::judge::DockerJudge;
use super::judge::Judge;
use super::GitTarget;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Stage {
    deadline: (time::Instant, time::Instant),
    judge: DockerJudge,
    path: PathBuf,
}

impl Stage {
    pub fn poll(&self, repo_url: String) -> Option<GitTarget> {
        if !(time::Instant::now() > self.deadline.0 && time::Instant::now() < self.deadline.1) {
            //TODO: proper logging
            return None;
        }

        // ./scripts/retard_polling.sh {repo_url} {self.path}
        let output = Command::new("./script/retard_polling.sh")
            .arg(repo_url)
            .arg(self.path)
            .output()
            .expect("failed to run retartd_polling script");
        match output.status.success() {
            true => {
                Some(GitTarget::repo(repo_url).on_commit(String::from_utf8(output.stdout).ok()?))
            }
            false => None,
        }
    }
    fn trigger(&self, target:GitTarget) -> Result<f64> {
        self.judge.judge(target, self.path)
    }
}
