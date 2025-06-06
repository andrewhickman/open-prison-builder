from typing import Optional

import numpy as np
import pb_learn_env
import gymnasium as gym

class PbEnvironment(gym.Env):
    def __init__(self, config: Optional[dict] = None):
        self.action_space = gym.spaces.Box(-1., 1., shape=(3,), dtype=np.float32)
        self.observation_space = gym.spaces.Box(-8., 8., shape=(12,), dtype=np.float32)
        self.env = pb_learn_env.Environment()

    def reset(self, *, seed=None, options=None):
        obs = self.env.reset(seed)
        return np.array(obs, np.float32), {}

    def step(self, action):
        obs, reward, terminated, truncated = self.env.step(action)
        infos = {}
        return (
            np.array(obs, np.float32),
            reward,
            terminated,
            truncated,
            infos,
        )
