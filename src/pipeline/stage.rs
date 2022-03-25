use std::{path::PathBuf, process::Command};

use super::error::Result;
use super::judge::DockerJudge;
use super::judge::Judge;
use super::GitTarget;
use chrono::TimeZone;
use chrono::{DateTime, Local};
use log::debug;
use serde::{de, Deserialize, Deserializer};

const POLLING_SCRIPT_FILEPATH: &str = "./scripts/retard_polling.sh";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Stage {
    pub name: String,

    // #[serde(deserialize_with = "parse_date_from_costume_string")]
    // deadline: (DateTime<Local>, DateTime<Local>),
    deadline: Deadline,
    judge: DockerJudge,
    path: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Deadline {
    #[serde(deserialize_with = "parse_date_from_costume_string")]
    from: DateTime<Local>,
    #[serde(deserialize_with = "parse_date_from_costume_string")]
    to: DateTime<Local>,
}

impl Stage {
    pub fn poll(&self, target: GitTarget) -> Option<GitTarget> {
        if !(Local::now() > self.deadline.from && Local::now() < self.deadline.to) {
            debug!(
                "datetime is out of bound for stage({}),aborting...",
                self.name
            );
            return None;
        }

        // ./scripts/retard_polling.sh {repo_url} {self.path}
        let output = Command::new(POLLING_SCRIPT_FILEPATH)
            .arg(&target.url)
            .arg(&self.path)
            .output()
            .expect("failed to run retartd_polling script");
        match output.status.success() {
            true => Some(target.on_commit(String::from_utf8(output.stdout).ok()?)),
            false => {
                debug!(
                    "{}",
                    String::from_utf8(output.stderr)
                        .unwrap_or("couldn't display command output".into())
                );
                None
            }
        }
    }
    pub fn trigger(&self, target: &GitTarget) -> Result<f64> {
        self.judge.judge(target, &self.path)
    }
}

fn parse_date_from_costume_string<'de, D>(
    deserializer: D,
) -> std::result::Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    Ok(Local.datetime_from_str(&s, "%Y_%m_%d-%H:%M:%S").unwrap())
}
