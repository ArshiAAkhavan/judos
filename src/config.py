import os
from dataclasses import dataclass
from typing import Dict, List, Set

import yaml


@dataclass
class Config:
    concurrency: int
    repos: List[str]
    stages: Set[Dict[str, str]]
    poll_interval: int
    scoreboard: Dict[str, str]
    name: str


configs: Config
with open("./config.yml", "r", encoding="utf-8") as stream:
    try:
        conf_dict = yaml.safe_load(stream)
        os.environ["GAY_LEVEL"] = conf_dict["log-level"]
        del conf_dict["log-level"]

        configs = Config(**conf_dict)
    except yaml.YAMLError as exc:
        print(exc)
        raise exc


def get_configs() -> Config:
    return configs
