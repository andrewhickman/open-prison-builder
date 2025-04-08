import gymnasium as gym
import numpy as np

from ray.rllib.algorithms.ppo import PPOConfig
from ray.rllib.core.rl_module.default_model_config import DefaultModelConfig
from ray.rllib.env.tcp_client_inference_env_runner import TcpClientInferenceEnvRunner

from tcp_client_inference_env_runner import TcpClientInferenceEnvRunner

# class MyRLModule(TorchRLModule):
#     def setup(self):
#         ...
#         # Set the following attribute at the end of your custom `setup()`.
#         self.action_dist_cls = YOUR_DIST_CLASS

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
    )
    .rl_module(model_config=DefaultModelConfig(fcnet_hiddens=[5], vf_share_layers=True))
)
ppo = config.build_algo()

for _ in range(4):
    ppo.train()


    # @OverrideToImplementCustomLogic
    # @override(RLModule)
    # def get_inference_action_dist_cls(self) -> Type[TorchDistribution]:
    #     if self.action_dist_cls is not None:
    #         return self.action_dist_cls
    #     elif isinstance(self.action_space, gym.spaces.Discrete):
    #         return TorchCategorical
    #     elif isinstance(self.action_space, gym.spaces.Box):
    #         return TorchDiagGaussian

    # def from_logits