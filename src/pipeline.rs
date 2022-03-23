use serde::Deserialize;
use std::process::Command;
use std::{path::PathBuf, time};

use crate::judge::DockerJudge;
//TODO: better place for CommitHash Type;
use crate::error::Result;
use crate::judge::CommitHash;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Pipeline {
    name: String,
    poll_interval: usize,
    log_level: String,
    scoreboard: Scoreboard,
    stages: Vec<Stage>,
    repos: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Scoreboard {
    repo: String,
    target: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Stage {
    deadline: (time::Instant, time::Instant),
    judge: DockerJudge,
    path: PathBuf,
}

impl Stage {
    fn poll(&self, repo_url: String) -> Option<CommitHash> {
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
            true => String::from_utf8(output.stdout).ok(),
            false => None,
        }
    }
    fn trigger(&self, repo_url: String, commit: CommitHash) -> Result<f64> {
        Ok(0.0)
    }
}
