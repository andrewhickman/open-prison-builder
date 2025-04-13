import gymnasium as gym
import numpy as np
from ray.rllib.algorithms.ppo import PPOConfig
import torch

from env import PbEnvironment;

config = (
    PPOConfig()
    .environment(
        PbEnvironment,
    )
)

ppo = config.build_algo()

for _ in range(200):
    res = ppo.train()
    print(f"training iteration {res['training_iteration']}")
    print(f"episode reward mean {res['env_runners'].get('episode_return_mean')}")

    if res['done']:
        break

torch.onnx.export(
    ppo.get_module(),
    {
        'batch': {
            'obs': torch.randn(1, 5)
        }
    },
    "model.pb",
)
