use serde::Deserialize;
use std::process::Command;
use std::{path::PathBuf, time};

use crate::judge::{DockerJudge, Judge};
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
    score_file: PathBuf,
    commit_file: PathBuf,
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
        self.judge.judge(repo_url, commit, self.path)
    }
}

impl Scoreboard {
    fn update_grade(&self, stage_name: String, repo_name: String, grade: f64, commit: CommitHash) {
        // ./scripts/update_scoreboard.sh {self.scoreboard["file_name"]} {self.scoreboard["repo"]} {student_id} {score} {stage.id+2}
        let output = Command::new("./scripts/update_scoreboard.sh")
            .arg(self.score_file)
            .arg(self.repo)
            .arg(repo_name)
            .arg(grade.to_string())
            .arg(stage_name)
            .output()
            .expect("unable to run scoarbord script");
    }
}
