from libultimate import Controller, Console
import gym
import time
import threading

action_list_default = [
]

class UltimateEnv(gym.Env):
    def __init__(self, console: Console, controller: Controller, hz=60, action_list=action_list_default):
        super().__init__()
        self.hz = hz
        self.action_space = gym.spaces.Discrete(len(action_list)) 
        self.console = console
        self.controller = controller
        self.gamestate = None
        self.prev_gamestate = None
        self.run()

    def run(self):
        thread = threading.Thread(target=self._stream_gamestate)
        thread.start()
        time.sleep(1)

    def _stream_gamestate(self):
        for gamestate in self.console.stream(hz=self.hz):
            self.prev_gamestate = self.gamestate
            self.gamestate = gamestate

    def _gamestate_to_observation(self, gamestate):
        return gamestate

    def reset(self, without_reset=False):
        if not without_reset:
            self.controller.mode.training.reset()
        observation = self._gamestate_to_observation(self.gamestate)
        self.controller.release_all()
        time.sleep(1)
        return observation

    def step(self, action):
        self.controller.act(action)
        interval = 60/self.hz * (1/60)
        time.sleep(interval)
        observation = self._gamestate_to_observation(self.gamestate)
        info = self.gamestate
        self.done = self._done(self.gamestate, self.prev_gamestate)
        reward = self._reward(self.done, self.gamestate, self.prev_gamestate)
        self.prev_gamestate = self.gamestate
        return observation, reward, self.done, info

    def render(self, mode='human', close=False):
        if mode == 'human':
            print("You can see screen at http://localhost:8081/vnc.html")
        else:
            print("GameState: {}".format(self.gamestate))

    def _done(self, gamestate, prev_gamestate):
        if (prev_gamestate.players[0].percent != 0 and gamestate.players[0].percent == 0) or (prev_gamestate.players[1].percent != 0 and gamestate.players[1].percent == 0):
            return True
        return False

    def _reward(self, done, gamestate, prev_gamestate):
        p1_diff_damage = gamestate.players[0].percent - prev_gamestate.players[0].percent
        p2_diff_damage = gamestate.players[1].percent - prev_gamestate.players[1].percent
        reward = p2_diff_damage - p1_diff_damage
        if done:
            if gamestate.players[0].percent == 0:
                reward = -1
            if gamestate.players[1].percent == 0:
                reward = 1
        return reward
