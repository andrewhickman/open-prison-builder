from ray.rllib.algorithms.ppo import PPOConfig
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
    )
    .training(
        clip_param=0.13635534897407453,
        entropy_coeff=0.01,
        gamma=0.6179520528893198,
        grad_clip=40,
        kl_coeff=0.3199837970510152,
        kl_target=0.007275780799800806,
        lambda_=0.6215037224883218,
        lr=0.0004407381097165218,
        minibatch_size=982,
        num_epochs=14,
        train_batch_size=15728,
        use_kl_loss=False,
        vf_clip_param=25.6,
        vf_loss_coeff=0.12592186144165154,
    )
    .rl_module(
        model_config=DefaultModelConfig(
            fcnet_hiddens=[32],
        )
    )
)

ppo = config.build_algo()
ppo.restore_from_path(f"{__file__}/../models/movement")

for i in range(400):
    res = ppo.train()
    print(f"training iteration {i}")
    print(f"episode reward mean {res['env_runners'].get('episode_return_mean')}")

    if res['done']:
        break

    # if i % 100 == 99:
    #     ppo.save_to_path(f"{__file__}/../models/movement")


ppo.save_to_path(f"{__file__}/../models/movement")

torch.onnx.export(
    ppo.get_module(),
    {
        'batch': {
            'obs': torch.randn(1, 12)
        }
    },
    "models/movement.onnx",
)
