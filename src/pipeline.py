import logging
import os
import subprocess
import time
from datetime import datetime
from threading import Lock, Thread
from typing import Dict, List, Set, Tuple

from src.config import Config


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
        p.log_level = configs.log_level
        p.repos = configs.repos
        p.poll_interval = configs.poll_interval
        p.name = configs.name
        p.scoreboard = configs.scoreboard
        for i, (name, stage_dict) in enumerate(configs.stages.items()):
            stage_dict["id"] = i + 2
            p.stages[name] = Stage(**stage_dict)
        return p

    def run(self):
        logging.warning("Running...")
        poll_thread = Thread(target=self.poll_all, daemon=False)
        logging.warning(f"PollingThread {poll_thread.getName()} running...")
        poll_thread.start()
        working_threads = []

        for i in range(self.concurrency):
            worker = Thread(target=self.try_judge, daemon=False)
            logging.warning(f"WorkingThread {worker.getName()} running...")
            worker.start()
            working_threads.append(worker)

        poll_thread.join()
        for thread in working_threads:
            thread.join()

    def poll_all(self):
        while True:
            logging.warning("polling...")
            if self.check_done():
                return
            for repo in self.repos:
                for stage in self.stages:
                    if not (repo, stage) in self.polled:
                        if self.stages[stage].poll(repo):
                            self.lock.acquire()
                            self.queue.append((repo, stage))
                            self.polled.add((repo, stage))
                            self.lock.release()
            time.sleep(self.poll_interval)

    def try_judge(self):
        while True:
            if self.check_done():
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

    def update_scoreboard(self, uid, score, stage_id):
        os.system(
            f"./scripts/update_scoreboard.sh "
            f'{self.name}.csv {self.scoreboard["repo"]} '
            f"{uid} {score} {stage_id}"
        )

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


class Stage:
    def __init__(self, **kwargs):
        self.image = kwargs["judge"]["image"]
        self.copy_to = kwargs["judge"]["copy_to"]
        self.result_path = kwargs["judge"]["result_path"]

        self.id = kwargs["id"]
        self.path = kwargs["path"]
        self.start = kwargs["date_limit"]["start"]
        self.start = datetime.strptime(self.start, "%Y-%m-%d").timestamp()
        self.end = kwargs["date_limit"]["end"]
        self.end = datetime.strptime(self.end, "%Y-%m-%d").timestamp()

    def poll(self, repo_url: str) -> bool:
        now = datetime.now().timestamp()
        if now <= self.end and now >= self.start:
            exit_code = os.system(f"./scripts/retard_polling.sh {repo_url} {self.path}")

            return exit_code == 0
        return False

    def trigger(self, repo_url: str):
        logging.warning(f"stage triggered on {repo_url} on path {self.path}")
        os.system(
            f"./scripts/judge.sh {self.image} {repo_url} {self.path} {self.copy_to} {self.result_path}"
        )
        process = subprocess.Popen(
            f"./scripts/judge.sh {self.image} {repo_url} {self.path}".split(),
            stdout=subprocess.PIPE,
        )
        out, _ = process.communicate()

        logging.info("done!")

        return out.decode("utf-8").strip()
