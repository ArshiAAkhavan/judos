# from hammer.src.pipeline import Pipeline
import logging
import signal

from src.config import get_configs
from src.pipeline import Pipeline


def main():
    try:
        p = Pipeline.parse_from(get_configs())
        signal.signal(signal.SIGINT, p.exit)
        # signal.pause()
        p.run()
    except KeyboardInterrupt:
        logging.warning("shutting down gracefully")
        p.exit()


if __name__ == "__main__":
    main()
