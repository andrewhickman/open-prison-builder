from ray.rllib.algorithms.ppo import PPOConfig

from env import PbEnvironment;

# Point your config to your custom env class:
config = (
    PPOConfig()
    .environment(
        PbEnvironment,
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
