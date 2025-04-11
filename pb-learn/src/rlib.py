import gymnasium as gym
import numpy as np

from ray.rllib.algorithms.ppo import PPOConfig
from ray.rllib.core.rl_module.default_model_config import DefaultModelConfig
from ray.rllib.env.tcp_client_inference_env_runner import TcpClientInferenceEnvRunner

from tcp_client_inference_env_runner import TcpClientInferenceEnvRunner

config = (
    PPOConfig()
    .environment(
        observation_space = gym.spaces.Box(-10., 10., (7,), np.float32),
        action_space = gym.spaces.Box(-1.0, 1.0, (3,), np.float32),
        env_config = { "port": 27308 }
    )
    .env_runners(
        env_runner_cls = TcpClientInferenceEnvRunner,
        sample_timeout_s = 1000000,
        num_env_runners = 0
    )
    .training(
        num_epochs=10,
        vf_loss_coeff=0.01,
        lr = 0.002,
        grad_clip=4,
    )
    .framework(
        torch_skip_nan_gradients = False
    )
    .rl_module(model_config=DefaultModelConfig(fcnet_hiddens=[16], vf_share_layers=True))
)
ppo = config.build_algo()

for _ in range(100):
    ppo.train()
    
    # print(ppo.save_to_path())
