mod error;
mod judge;
mod scoreboard;
mod stage;
use std::{thread, time};

use crossbeam::{channel, select};
use serde::Deserialize;

//TODO: better place for CommitHash Type;
use judge::GitTarget;
use scoreboard::Scoreboard;
use stage::Stage;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
// struct Pipeline<'a> {
struct Pipeline {
    name: String,
    poll_interval: usize,
    concurrency: usize,
    log_level: String,
    scoreboard: Scoreboard,
    stages: Vec<Stage>,
    repos: Vec<String>,
    // #[serde(skip, default)]
    // work_queue: Vec<Work<'a>>,
}

#[derive(Debug)]
struct Work<'a> {
    repo_url: GitTarget,
    stage: &'a Stage,
}
impl<'a> Work<'a> {
    fn new(repo_url: String, stage: &'a Stage) -> Self {
        Self { repo_url, stage }
    }
}

impl Pipeline {
    pub fn run(&self) {
        let (wtx, wrx) = channel::unbounded();
        let (ptx, prx) = channel::unbounded();
        let (stx, srx) = channel::bounded(self.concurrency + 1);

        let handles = Vec::new();
        // poll_all thread
        let srx_pollall = srx.clone();
        handles.push(thread::spawn(move || {
            let interval = channel::tick(time::Duration::from_secs(1));
            loop {
                select! {
                    recv(interval) -> _ticked => {
                        for repo in self.repos{
                            for stage in self.stages{
                                ptx.send(Work::new(repo.clone(),&stage));
                            }
                        }
                    },
                    recv(srx_pollall) -> _sig => {
                        eprintln!("poll_all thread recieved exit signal, exiting")
                    }
                }
            }
        }));
        for i in 1..=self.concurrency {
            let srx = srx.clone();
            let prx = prx.clone();
            let (wrx, wtx) = (wrx.clone(), wtx.clone());
            handles.push(thread::spawn(move || loop {
                select! {
                    recv(srx) -> _sig => {
                        eprintln!("thread {i} recieved exit signal, exiting...");
                        return;
                    },
                    recv(prx) -> work => {
                    }

                }
            }));
            let work = prx.recv();
            let Work { repo_url, stage } = work.unwrap();
            let res = stage.poll(repo_url);
        }
    }
}
