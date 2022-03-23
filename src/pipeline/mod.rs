mod error;
mod judge;
mod scoreboard;
mod stage;
use serde::Deserialize;

//TODO: better place for CommitHash Type;
use judge::CommitHash;
use scoreboard::Scoreboard;
use stage::Stage;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Pipeline<'a> {
    name: String,
    poll_interval: usize,
    log_level: String,
    scoreboard: Scoreboard,
    stages: Vec<Stage>,
    repos: Vec<String>,

    #[serde(skip, default)]
    work_queue: Vec<Work<'a>>,
}

#[derive(Debug)]
struct Work<'a> {
    repo_url: String,
    stage: &'a Stage,
}

impl Pipeline {}
