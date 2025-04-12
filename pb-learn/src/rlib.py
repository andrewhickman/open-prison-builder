import random
from typing import Optional
import gymnasium as gym
import numpy as np

from ray.rllib.algorithms.ppo import PPOConfig

# These tags allow extracting portions of this script on Anyscale.
# ws-template-code-start
class SimpleCorridor(gym.Env):
    """Example of a custom env in which the agent has to walk down a corridor.

    ------------
    |S........G|
    ------------
    , where S is the starting position, G is the goal position, and fields with '.'
    mark free spaces, over which the agent may step. The length of the above example
    corridor is 10.
    Allowed actions are left (0) and right (1).
    The reward function is -0.01 per step taken and a uniform random value between
    0.5 and 1.5 when reaching the goal state.

    You can configure the length of the corridor via the env's config. Thus, in your
    AlgorithmConfig, you can do:
    `config.environment(env_config={"corridor_length": ..})`.
    """

    def __init__(self, config: Optional[dict] = None):
        config = config or {}
        self.end_pos = config.get("corridor_length", 7)
        self.cur_pos = 0
        self.action_space = gym.spaces.Discrete(2)
        self.observation_space = gym.spaces.Box(0.0, self.end_pos, shape=(1,), dtype=np.float32)

    def reset(self, *, seed=None, options=None):
        random.seed(seed)
        self.cur_pos = 0
        # Return obs and (empty) info dict.
        return np.array([self.cur_pos], np.float32), {"env_state": "reset"}

    def step(self, action):
        assert action in [0, 1], action
        # Move left.
        if action == 0 and self.cur_pos > 0:
            self.cur_pos -= 1
        # Move right.
        elif action == 1:
            self.cur_pos += 1

        # The environment only ever terminates when we reach the goal state.
        terminated = self.cur_pos >= self.end_pos
        truncated = False
        # Produce a random reward from [0.5, 1.5] when we reach the goal.
        reward = random.uniform(0.5, 1.5) if terminated else -0.01
        infos = {}
        return (
            np.array([self.cur_pos], np.float32),
            reward,
            terminated,
            truncated,
            infos,
        )

# Point your config to your custom env class:
config = (
    PPOConfig()
    .environment(
        SimpleCorridor,  # or provide the registered string: "corridor-env"
        env_config={"corridor_length": 10},
    )
)

# Build a PPO algorithm and train it.
ppo_w_custom_env = config.build_algo()

while True:
    res = ppo_w_custom_env.train()
    print(f"training iteration {res['training_iteration']}")
    print(f"episode reward mean {res['env_runners']['episode_return_mean']}")

    if res['done']:
        break
