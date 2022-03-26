use std::{path::PathBuf, process::Command};

use super::judge::GitTarget;
use log::{debug, info, warn};
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
        let output = Command::new(UPDATE_SCOREBOARD_SCRIPT_FILE_PATH)
            .arg(&self.scorefile)
            .arg(&self.repo)
            .arg(target.get_name())
            .arg(format!("{grade:.1}"))
            .arg(stage_name)
            .arg(&self.commitfile)
            .arg(&target.commit)
            .output()
            .expect("unable to run scoarbord script");

        match output.status.success() {
            true => {
                info!("update score {grade:.1} for {target} on stage({stage_name})");
            }
            false => {
                warn!("failed to update score {grade:.1} for {target} on stage({stage_name})");
            }
        }
        debug!(
            "{}",
            String::from_utf8(output.stdout).unwrap_or("stdout can not be displayed".into())
        );
        debug!(
            "{}",
            String::from_utf8(output.stderr).unwrap_or("stderr can not be displayed".into())
        );
    }
}
