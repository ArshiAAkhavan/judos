import signal

from src.config import get_configs
from src.pipeline import Pipeline


def main():
    p = Pipeline.parse_from(get_configs())
    signal.signal(signal.SIGINT, p.exit)
    p.run()


if __name__ == "__main__":
    main()
