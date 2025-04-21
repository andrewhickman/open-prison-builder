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
        clip_param=0.18136109057935812,
        entropy_coeff=0.0,
        gamma=0.9714908573540817,
        grad_clip=40,
        kl_coeff=0.3998538748809515,
        kl_target=0.028076881805112107,
        lambda_=0.9912937250361135,
        lr=0.000623028408669572,
        minibatch_size=1024,
        num_epochs=13,
        train_batch_size=31744,
        use_kl_loss=True,
        vf_clip_param=40.0,
        vf_loss_coeff=0.5390734273333284,
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
