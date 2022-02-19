import logging
import os
import time
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

    @staticmethod
    def parse_from(configs: Config) -> 'Pipeline':
        p = Pipeline()
        p.concurrency = configs.concurrency
        p.log_level = configs.log_level
        p.repos = configs.repos
        for (name, stage_dict) in configs.stages.items():
            p.stages[name] = Stage(**stage_dict)
        return p

    def run(self):
        poll_thread = Thread(target=self.poll_all, daemon=False)
        poll_thread.start()
        working_threads = []
        for i in range(self.concurrency):
            worker = Thread(target=self.try_judge, daemon=False)
            worker.start()
            working_threads.append(worker)

        poll_thread.join()
        for thread in working_threads:
            thread.join()

    def poll_all(self):
        while True:
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
            time.sleep(1)

    def try_judge(self):
        while True:
            if self.check_done():
                return
            if len(self.queue):
                self.lock.acquire()
                if len(self.queue):
                    (repo, stage) = self.queue.pop(0)
                    self.polled.remove((repo, stage))
                self.lock.release()
                self.stages[stage].trigger(repo)
            else:
                time.sleep(1)

    def exit(self):
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
        self.path = kwargs['path']
        self.image = kwargs['image']

    def poll(self, repo_url: str) -> bool:
        exit_code = os.system(f'./scripts/retard_polling.sh {repo_url} {self.path}')
        return exit_code == 0

    def trigger(self, repo_url: str):
        logging.warning(f"stage triggered on {repo_url} on path {self.path}")
        os.system(f'./scripts/judge.sh {self.image} {repo_url} {self.path}')
        logging.info("done!")
