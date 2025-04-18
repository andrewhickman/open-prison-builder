import pprint
import random
from ray import tune
from ray.rllib.algorithms.ppo import PPOConfig
from ray.rllib.core.rl_module.default_model_config import DefaultModelConfig
from ray.tune.schedulers import PopulationBasedTraining

import torch

from env import PbEnvironment;

def explore(config):
    if config["train_batch_size"] < config["sgd_minibatch_size"] * 2:
        config["train_batch_size"] = config["sgd_minibatch_size"] * 2
    return config

hyperparam_mutations = {
    "clip_param": lambda: random.uniform(0.01, 0.5),
    "lr": [1e-3, 5e-4, 1e-4, 5e-5, 1e-5],
    "num_epochs": lambda: random.randint(1, 30),
    "minibatch_size": lambda: random.randint(128, 16384),
    "train_batch_size_per_learner": lambda: random.randint(2000, 160000),
}

pbt = PopulationBasedTraining(
    time_attr="time_total_s",
    perturbation_interval=120,
    resample_probability=0.25,
    # Specifies the mutations of these hyperparams
    hyperparam_mutations=hyperparam_mutations,
    custom_explore_fn=explore,
    require_attrs=False,
)

# Stop when we've either reached 100 training iterations or reward=300
stopping_criteria = {"training_iteration": 100, "env_runners/episode_return_mean": 5}

#{'clip_param': 0.192, 'lr': 5e-05, 'minibatch_size': 1310, 'num_epochs': 19}

config = (
    PPOConfig()
    .environment(
        PbEnvironment,
    )
    .env_runners(
        num_env_runners=16
    )
    .training(
        # These params are tuned from a fixed starting value.
        kl_coeff=1.0,
        lambda_=0.95,
        clip_param=0.2,
        lr=1e-4,
        # These params start off randomly drawn from a set.
        num_epochs=tune.choice([10, 20, 30]),
        minibatch_size=tune.choice([128, 512, 2048]),
        train_batch_size_per_learner=tune.choice([10000, 20000, 40000]),
    )
    .rl_module(
        model_config=DefaultModelConfig(
            fcnet_hiddens=[32],
        )
    )
)

tuner = tune.Tuner(
    "PPO",
    tune_config=tune.TuneConfig(
        metric="env_runners/episode_return_mean",
        mode="max",
        scheduler=pbt,
        num_samples=2,
    ),
    param_space=config,
    run_config=tune.RunConfig(stop=stopping_criteria),
)
results = tuner.fit()

print(results)

best_result = results.get_best_result()

print("Best performing trial's final set of hyperparameters:\n")
pprint.pprint(
    {k: v for k, v in best_result.config.items() if k in hyperparam_mutations}
)

print("\nBest performing trial's final reported metrics:\n")

metrics_to_print = [
    "episode_return_mean",
    "episode_reward_max",
    "episode_reward_min",
    "episode_len_mean",
]
pprint.pprint({k: v for k, v in best_result.metrics.items() if k in metrics_to_print})

print(best_result.checkpoint)

# for _ in range(200):
#     res = ppo.train()
#     print(f"training iteration {res['training_iteration']}")
#     print(f"episode reward mean {res['env_runners'].get('episode_return_mean')}")

#     if res['done']:
#         break

from ray.rllib.algorithms.algorithm import Algorithm

loaded_ppo = Algorithm.from_checkpoint(best_result.checkpoint)

torch.onnx.export(
    loaded_ppo.get_module(),
    {
        'batch': {
            'obs': torch.randn(1, 5)
        }
    },
    "models/movement.onnx",
)
