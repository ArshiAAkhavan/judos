use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

pub type CommitHash = String;

pub trait Judge {
    fn judge(&self, repo_url: String, from_path: PathBuf);
}

#[derive(Debug, Deserialize)]
pub struct DockerJudge {
    image: String,
    copy_to: PathBuf,
    result_path: PathBuf,
}

impl Judge for DockerJudge {
    fn judge(&self, repo_url: String, from_path: PathBuf) {
        // ./scripts/judge.sh {self.image} {repo_url} {self.path} {self.copy_to} {self.result_path}
        let output = Command::new("./scripts/judge.sh")
            .arg(self.image)
            .arg(repo_url)
            .arg(from_path)
            .arg(self.copy_to)
            .arg(self.result_path)
            .output()
            .expect("failed to execute judge script");
        output.output;
    }
}
