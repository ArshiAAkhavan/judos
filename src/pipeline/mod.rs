mod error;
mod judge;
mod scoreboard;
mod stage;
use std::{thread, time};

use crossbeam::{self, channel, select};
use serde::Deserialize;

//TODO: better place for CommitHash Type;
use judge::GitTarget;
use scoreboard::Scoreboard;
use stage::Stage;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Pipeline {
    name: String,
    poll_interval: u32,
    concurrency: usize,
    log_level: String,
    scoreboard: Scoreboard,
    stages: Vec<Stage>,
    repos: Vec<String>,
}

#[derive(Debug)]
struct Work<'a> {
    target: GitTarget,
    stage: &'a Stage,
}
impl<'a> Work<'a> {
    fn new(target: GitTarget, stage: &'a Stage) -> Self {
        Self { target, stage }
    }
}

impl Pipeline {
    pub fn run(&self) {
        let (wtx, wrx) = channel::unbounded();
        let (ptx, prx) = channel::unbounded();
        // TODO: handle exit signal
        let (_stx, srx) = channel::bounded::<()>(self.concurrency + 1);

        // poll_all thread
        let srx_pollall = srx.clone();
        crossbeam::scope(|s| {
            s.spawn(|_| {
                let interval = channel::tick(time::Duration::from_secs(self.poll_interval as u64));
                loop {
                    select! {
                        recv(interval) -> _ticked => {
                            for repo in &self.repos{
                                for stage in &self.stages{
                                    ptx.send(Work::new(GitTarget::repo(repo.clone()),&stage)).unwrap();
                                }
                            }
                        },
                        recv(srx_pollall) -> _sig => {
                            eprintln!("poll_all thread recieved exit signal, exiting")
                        }
                    }
                }
            });
            for i in 1..=self.concurrency {
                let srx = srx.clone();
                let prx = prx.clone();
                let (wtx, wrx) = (wtx.clone(), wrx.clone());
                s.spawn(move |_| loop {
                    select! {
                        recv(srx) -> _sig => {
                            eprintln!("thread {i} recieved exit signal, exiting...");
                            return;
                        },
                        recv(prx) -> work => {
                            let Work { target, stage } = work.unwrap();
                            // TODO: check for duplicate polled
                            match stage.poll(target) {
                                Some(target) => wtx.send(Work::new(target, stage)).unwrap(),
                                None => (),
                            }
                        }
                        recv(wrx) -> work => {
                            let Work { target, stage } = work.unwrap();
                            match stage.trigger(&target) {
                                Ok(grade) => {
                                    self.scoreboard.update_grade(&stage.name, &target, grade);
                                },
                                // TODO: better logging
                                Err(e) => {eprintln!("{e:?}")},
                            }
                        }
                    }
                });
            }
        })
        .unwrap();
    }
}
