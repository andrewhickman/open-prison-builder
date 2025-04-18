from ray.rllib.algorithms.ppo import PPOConfig
from ray.rllib.algorithms.algorithm import Algorithm
from ray.rllib.core.rl_module.default_model_config import DefaultModelConfig

import torch

from env import PbEnvironment;

config = (
    PPOConfig()
    .environment(
        PbEnvironment,
    )
    .env_runners(
        num_env_runners=31
        # num_env_runners=0
    )
    .training(
        kl_coeff=0.5,
        gamma=0.95,
        lambda_=0.97,
        clip_param=0.192,
        lr=5e-05,
        num_epochs=19,
        minibatch_size=1310,
    )
    .rl_module(
        model_config=DefaultModelConfig(
            fcnet_hiddens=[24],
        )
    )
)

# ppo = Algorithm.from_checkpoint('C:/Users/andre/ray_results/PPO_2025-04-18_14-35-29/PPO_PbEnvironment_043fc_00001_1_train_batch_size_per_learner=40000_2025-04-18_14-35-39/checkpoint_000005')
ppo = config.build_algo()
ppo.restore_from_path(f"{__file__}/../models/movement")

for i in range(500):
    res = ppo.train()
    print(f"training iteration {i}")
    print(f"episode reward mean {res['env_runners'].get('episode_return_mean')}")

    if res['done']:
        break

ppo.save_to_path(f"{__file__}/../models/movement")

torch.onnx.export(
    ppo.get_module(),
    {
        'batch': {
            'obs': torch.randn(1, 5)
        }
    },
    "models/movement.onnx",
)
