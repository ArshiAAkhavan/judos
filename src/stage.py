import os
import subprocess
from datetime import datetime

import gaylogger as glogging

logger = glogging.getLogger("stage")


class Stage:
    def __init__(self, **kwargs):

        # judge spec
        self.image = kwargs["judge"]["image"]
        self.copy_to = kwargs["judge"]["copy_to"]
        self.result_path = kwargs["judge"]["result_path"]

        # stage attr spec
        self.name = kwargs["name"]
        self.id = kwargs["id"]
        self.path = kwargs["path"]

        # time limit spec
        self.start = kwargs["deadline"]["start"]
        self.start = datetime.strptime(self.start, "%Y-%m-%d").timestamp()
        self.end = kwargs["deadline"]["end"]
        self.end = datetime.strptime(self.end, "%Y-%m-%d").timestamp()

    def poll(self, repo_url: str) -> bool:
        logger.debug(f"start polling for Stage::{self.name}({repo_url}/{self.path})")
        now = datetime.now().timestamp()
        if now <= self.end and now >= self.start:
            process = subprocess.Popen(
                f"./scripts/retard_polling.sh {repo_url} {self.path}".split(),
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            out, err = process.communicate()
            exit_code = process.returncode
            logger.debug(out)
            logger.debug(err)
            return exit_code == 0
        logger.debug(
            f"polling Stage::{self.name}({repo_url}/{self.path}) aborted, date out of bound for this stage"
        )
        return False

    def trigger(self, repo_url: str) -> (str, float):
        logger.info(f"stage triggered on Stage::{self.name}({repo_url}/{self.path})")
        process = subprocess.Popen(
            f"./scripts/judge.sh {self.image} {repo_url} {self.path} {self.copy_to} {self.result_path}".split(),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        out, err = process.communicate()

        logger.debug(
            "output result for trigger on Stage::{self.name}({repo_url}/{self.path})"
        )
        logger.debug(err)
        commit_hash, grade = out.decode("utf-8").strip().split()[:2]
        float_grade = 0.0
        try:
            float_grade = float(grade)
        except Exception:
            logger.warning(f"output grade wasn't parsable, using {float_grade} instead")

        logger.info(
            f"grade for Stage::{self.name}({repo_url}/{self.path}) is {float_grade}"
        )
        return (commit_hash, float_grade)
