# from hammer.src.pipeline import Pipeline
import logging

from src.config import get_configs
from src.pipeline import Pipeline


def main():
    try:
        p = Pipeline.parse_from(get_configs())
        p.run()
    except KeyboardInterrupt:
        logging.warning("shutting down gracefully")
        p.exit()


if __name__ == "__main__":
    main()
