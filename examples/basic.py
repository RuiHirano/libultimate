import os
from libultimate import Console, Controller, Button

if __name__ == "__main__":
    RYUJINX_PATH = os.path.join(os.path.dirname(__file__), "../libultimate/test")
    console = Console(ryujinx_path=RYUJINX_PATH)
    controller_1p = Controller(console, player_id=1)

    for gamestate in console.stream(hz=5):
        print("gamestate: ", gamestate)
        controller_1p.act(Button.A)
