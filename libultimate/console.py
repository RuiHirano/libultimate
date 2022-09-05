import os
import sys
import time
import logging
sys.path.append(os.path.join(os.path.dirname(__file__), '..'))
from .api import API

class Console():
    def __init__(self, ryujinx_path, level=logging.WARNING):
        logging.basicConfig(format='%(asctime)s - [%(levelname)s] - %(message)s', level=level)
        self.logger = logging.getLogger(__name__)
        self.ryujinx_path = ryujinx_path
        self.api = API(ryujinx_path)

    def run(self):
        pass

    def stream(self, hz=60):
        interval = 60/hz * (1/60)
        while True:
            try:
                time.sleep(interval)
                yield self.api.read_state()
            except Exception as err:
                self.logger.warning("couldn't read state: {}".format(err))

if __name__ == "__main__":
    RYUJINX_PATH = os.path.join(os.path.dirname(__file__), "test")
    console = Console(RYUJINX_PATH)
    for game_state in console.stream(hz=5):
        print(game_state)
