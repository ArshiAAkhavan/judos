use std::{path::PathBuf, process::Command};

use super::judge::GitTarget;
use log::{info, warn,debug};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Scoreboard {
    repo: String,
    scorefile: PathBuf,
    commitfile: PathBuf,
}

const UPDATE_SCOREBOARD_SCRIPT_FILE_PATH: &str = "./scripts/update_scoreboard.sh";
impl Scoreboard {
    pub fn update_grade(&self, stage_name: &str, target: &GitTarget, grade: f64) {
        // ./scripts/update_scoreboard.sh {self.scoreboard["file_name"]} {self.scoreboard["repo"]} {student_id} {score} {stage.id+2}
        // TODO: use commitHash
        // TODO: handle stage_name (previous implementation used stage_id)
        let output = Command::new(UPDATE_SCOREBOARD_SCRIPT_FILE_PATH)
            .arg(&self.scorefile)
            .arg(&self.repo)
            .arg(target.get_name())
            .arg(grade.to_string())
            .arg(stage_name)
            .output()
            .expect("unable to run scoarbord script");

        match output.status.success() {
            true => {
                info!("update score {grade:.1} for {target} on stage({stage_name})");
                info!("{}",String::from_utf8(output.stdout).unwrap_or("stdout can not be displayed".into()));
                info!("{}",String::from_utf8(output.stderr).unwrap_or("stderr can not be displayed".into()));
            }
            false => {
                warn!("failed to update score {grade:.1} for {target} on stage({stage_name})");
                debug!("{}",String::from_utf8(output.stdout).unwrap_or("stdout can not be displayed".into()));
                debug!("{}",String::from_utf8(output.stderr).unwrap_or("stderr can not be displayed".into()));
            }
        }
    }
}
