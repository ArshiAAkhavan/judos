use std::{path::PathBuf, process::Command};

use serde::Deserialize;
use log::info;
use super::judge::GitTarget;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Scoreboard {
    repo: String,
    scorefile: PathBuf,
    commitfile: PathBuf,
}

impl Scoreboard {
    pub fn update_grade(&self, stage_name: &str, target: &GitTarget, grade: f64) {
        // ./scripts/update_scoreboard.sh {self.scoreboard["file_name"]} {self.scoreboard["repo"]} {student_id} {score} {stage.id+2}
        // TODO: use commitHash
        let _output = Command::new("./scripts/update_scoreboard.sh")
            .arg(&self.scorefile)
            .arg(&self.repo)
            .arg(target.get_name())
            .arg(grade.to_string())
            .arg(stage_name)
            .output()
            .expect("unable to run scoarbord script");
        info!("update score {grade:.1} for {target} on stage({stage_name})");
        
    }
}
