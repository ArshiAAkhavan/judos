import os
import subprocess
import time
from threading import Lock, Thread
from typing import Dict, List, Set, Tuple

import gaylogger as glogging
from src.config import Config
from src.stage import Stage

logger = glogging.getLogger("pipeline")


class Pipeline:
    def __init__(self):
        self.queue: List[Tuple[str, str]] = []
        self.done: List[int] = []
        self.polled: Set[Tuple[str, str]] = set()
        self.concurrency = None
        self.log_level = None
        self.repos = []
        self.stages: Dict[str, Stage] = {}
        self.lock = Lock()
        self.poll_interval = None
        self.name = None
        self.scoreboard = None

    @staticmethod
    def parse_from(configs: Config) -> "Pipeline":
        p = Pipeline()
        p.concurrency = configs.concurrency
        p.repos = configs.repos
        p.poll_interval = configs.poll_interval
        p.name = configs.name
        p.scoreboard = configs.scoreboard
        for i, (name, stage_dict) in enumerate(configs.stages.items()):
            stage_dict["id"] = i
            p.stages[name] = Stage(**stage_dict)
        return p

    def run(self):
        logger.info("Running...")
        poll_thread = Thread(target=self.poll_all, daemon=False)
        logger.info(f"PollingThread {poll_thread.getName()} running...")
        poll_thread.start()
        working_threads = []

        for i in range(self.concurrency):
            worker = Thread(target=self.try_judge, daemon=False)
            logger.info(f"WorkingThread {worker.getName()} running...")
            worker.start()
            working_threads.append(worker)

        poll_thread.join()
        for thread in working_threads:
            thread.join()

    def poll_all(self):
        while True:
            logger.info("polling...")
            if self.check_done():
                logger.warning(
                    "PollingThread recieved exit signal, exiting...")
                return
            for repo in self.repos:
                for stage in self.stages:
                    if not (repo, stage) in self.polled:
                        if self.stages[stage].poll(repo):
                            self.lock.acquire()
                            self.queue.append((repo, stage))
                            self.polled.add((repo, stage))
                            self.lock.release()
                            logger.info(f"{(repo,stage)} added to queue")
            time.sleep(self.poll_interval)

    def try_judge(self):
        while True:
            if self.check_done():
                logger.warning("WorkerThread recieved exit signal, exiting...")
                return
            if len(self.queue):
                self.lock.acquire()
                if len(self.queue):
                    (repo, stage) = self.queue.pop(0)
                    self.lock.release()

                    uid = repo.split("/")[-1]
                    score = self.stages[stage].trigger(repo)
                    self.update_scoreboard(uid, score, self.stages[stage].id)

                    self.lock.acquire()
                    self.polled.remove((repo, stage))
                self.lock.release()
            else:
                time.sleep(self.poll_interval)

    def update_scoreboard(self, student_id, score, stage_id):
        logger.info(
            f"update score for {student_id} with score {score} in stage[{stage_id}]"
        )

        cmd = f'./scripts/update_scoreboard.sh {self.name}.csv {self.scoreboard["repo"]} {student_id} {score} {stage_id+2}'
        process = subprocess.Popen(
            cmd.split(),
            stdout=subprocess.PIPE,
            # stderr=subprocess.PIPE,
        )
        out, err = process.communicate()
        logger.info(
            f"updated score for {student_id} with score {score} in stage[{stage_id}]"
        )
        # logger.debug()

    def exit(self, signal, frame):
        self.lock.acquire()
        for _ in range(self.concurrency + 1):
            self.done.append(0)
        self.lock.release()
        while True:
            self.lock.acquire()
            if len(self.done) == 0:
                break
            self.lock.release()
        self.lock.release()
        return

    def check_done(self) -> bool:
        if len(self.done):
            self.lock.acquire()
            if len(self.done):
                self.done.pop()
                self.lock.release()
                return True
            self.lock.release()
        return False
