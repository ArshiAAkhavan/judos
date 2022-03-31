mod error;
mod judge;
mod scoreboard;
mod stage;
use std::{fmt::Display, time};

use chrono::Local;
use crossbeam::{
    self,
    channel::{self, Receiver, Sender},
    select,
};
use log::{debug, error, info, warn};
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
impl<'a> Display for Work<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Work<{},{}>", self.stage.name, self.target)
    }
}

impl Pipeline {
    pub fn run(&self, sig_in: Receiver<()>) {
        let (wtx, wrx) = channel::unbounded();
        let (ptx, prx) = channel::unbounded();
        let (stx, srx) = channel::bounded::<()>(self.concurrency + 1);

        // poll_all thread
        let srx_pollall = srx.clone();
        crossbeam::scope(|s| {
            info!("spawning signal handler thread");
            s.spawn(|_| {
                select! {
                    recv(sig_in) -> _signal => {
                        warn!("pipeline received exit signal, propagating...");
                        for _ in 0..stx.capacity().unwrap(){
                            stx.send(()).expect("couldn't send SIGINT to all threads, panic!");
                        }
                    },
                };
            });
            info!("spawning the poll_all thread");
            s.spawn(|_| {
                let interval = channel::tick(time::Duration::from_secs(self.poll_interval as u64));
                loop {
                    select! {
                        recv(interval) -> _ticked => {
                            info!("polling...");
                            self.poll_all(&ptx);
                        },
                        recv(srx_pollall) -> _sig => {
                            error!("poll_all thread recieved exit signal, exiting");
                            return;
                        }
                    }
                }
            });
            for i in 1..=self.concurrency {
                let srx = srx.clone();
                let prx = prx.clone();
                let (wtx, wrx) = (wtx.clone(), wrx.clone());
                info!("spawning working thread no.{i}");
                s.spawn(move |_| loop {
                    select! {
                        recv(srx) -> _sig => {
                            error!("worker {i} recieved exit signal, exiting...");
                            return;
                        },
                        recv(prx) -> work => {
                            let work = work.unwrap();
                            debug!("thread {i} received [poll] order on {work}");
                            self.poll(work,&wtx);
                        }
                        recv(wrx) -> work => {
                            let work = work.unwrap();
                            info!("worker {i} recieved [judge] order on {work}");
                            self.judge(work);
                        }
                    }
                });
            }
        })
        .unwrap();
    }
    fn poll<'a>(&self, work: Work<'a>, wtx: &Sender<Work<'a>>) {
        let Work { target, stage } = work;
        if let Some(target) = stage.poll(target) {
            info!(
                "poll resulted in ({},{target}), pushing to work queue...",
                stage.name
            );
            wtx.send(Work::new(target, stage)).unwrap()
        };
    }
    fn judge(&self, work: Work<'_>) {
        let Work { target, stage } = work;
        match stage.trigger(&target) {
            Ok(grade) => {
                println!(
                    "[{}] {target} received score {grade:.1} for stage {}",
                    Local::now(),
                    stage.name
                );
                self.scoreboard.update_grade(&stage.name, &target, grade);
            }
            Err(e) => {
                error!("judge failed to run with the following error{e:?}")
            }
        }
    }
    fn poll_all<'a>(&'a self, ptx: &Sender<Work<'a>>) {
        // TODO: handle duplication better
        if !ptx.is_empty() {
            debug!("last poll is not finished yet!");
            return;
        }
        for repo in &self.repos {
            for stage in &self.stages {
                debug!("marking ({repo},{}) as a candidate", stage.name);
                ptx.send(Work::new(GitTarget::repo(repo.clone()), stage))
                    .unwrap();
            }
        }
    }
}
