import os
import subprocess
from datetime import datetime

import gaylogger as glogging

logger = glogging.getLogger("stage")


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
        logger.debug(f"start polling for {self.path} on repo {repo_url}")
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
            f"polling [{repo_url}/{self.path}] aborted, date out of bound for this stage"
        )
        return False

    def trigger(self, repo_url: str) -> float:
        logger.info(f"stage triggered on {repo_url} on path {self.path}")
        process = subprocess.Popen(
            f"./scripts/judge.sh {self.image} {repo_url} {self.path} {self.copy_to} {self.result_path}".split(),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        out, err = process.communicate()

        logger.debug("output result for trigger on [{repo_url}/{self.path}]")
        logger.debug(err)
        grade = out.decode("utf-8").strip()
        float_grade = 0.0
        try:
            float_grade = float(grade)
        except:
            logger.warning(
                f"output grade wasn't parsable, using {float_grade} instead")

        logger.info(f"grade for [{repo_url}/{self.path}] is {float_grade}")
        return float_grade
