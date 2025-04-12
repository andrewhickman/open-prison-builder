import random
from typing import Optional

import numpy as np
import pb_learn
import gymnasium as gym

class PbEnvironment(gym.Env):
    def __init__(self, config: Optional[dict] = None):
        self.action_space = gym.spaces.Discrete(2)
        self.observation_space = gym.spaces.Box(0.0, 10, shape=(1,), dtype=np.float32)
        self.env = pb_learn.Environment()

    def reset(self, *, seed=None, options=None):
        cur_pos = self.env.reset(seed)
        return np.array([cur_pos], np.float32), {"env_state": "reset"}

    def step(self, action):
        cur_pos, reward, terminated, truncated = self.env.step(action)
        infos = {}
        return (
            np.array([cur_pos], np.float32),
            reward,
            terminated,
            truncated,
            infos,
        )
