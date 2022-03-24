use std::{path::PathBuf, process::Command};

use super::CommitHash;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Scoreboard {
    repo: String,
    score_file: PathBuf,
    commit_file: PathBuf,
}

impl Scoreboard {
    pub fn update_grade(
        &self,
        stage_name: String,
        repo_name: String,
        grade: f64,
        commit: CommitHash,
    ) {
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
