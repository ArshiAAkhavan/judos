use std::{path::PathBuf, process::Command};

use super::judge::GitTarget;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Scoreboard {
    repo: String,
    score_file: PathBuf,
    commit_file: PathBuf,
}

impl Scoreboard {
    pub fn update_grade(&self, stage_name: &str, target: &GitTarget, grade: f64) {
        // ./scripts/update_scoreboard.sh {self.scoreboard["file_name"]} {self.scoreboard["repo"]} {student_id} {score} {stage.id+2}
        // TODO: use commitHash
        let _output = Command::new("./scripts/update_scoreboard.sh")
            .arg(&self.score_file)
            .arg(&self.repo)
            .arg(target.get_name())
            .arg(grade.to_string())
            .arg(stage_name)
            .output()
            .expect("unable to run scoarbord script");
    }
}
