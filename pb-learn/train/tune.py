import pprint
from ray import tune
from ray.tune.schedulers.pb2 import PB2
from ray.rllib.algorithms.ppo.ppo import PPOConfig
from ray.rllib.core.rl_module.default_model_config import DefaultModelConfig
from ray.rllib.utils.metrics import NUM_ENV_STEPS_SAMPLED_LIFETIME

import torch

from env import PbEnvironment;

def explore(config):
    if config["train_batch_size"] < config["sgd_minibatch_size"] * 2:
        config["train_batch_size"] = config["sgd_minibatch_size"] * 2
    return config

pb2_scheduler = PB2(
    time_attr=f"{NUM_ENV_STEPS_SAMPLED_LIFETIME}",
    metric="env_runners/episode_return_mean",
    mode="max",
    perturbation_interval=50000,
    # Copy bottom % with top % weights.
    quantile_fraction=0.25,
    hyperparam_bounds={
        "lr": [1e-5, 1e-3],
        "gamma": [0.95, 0.99],
        "lambda": [0.97, 1.0],
        "entropy_coeff": [0.0, 0.01],
        "vf_loss_coeff": [0.01, 1.0],
        "clip_param": [0.1, 0.3],
        "kl_target": [0.01, 0.03],
        "minibatch_size": [512, 4096],
        "num_epochs": [6, 32],
        "vf_share_layers": [False, True],
        "use_kl_loss": [False, True],
        "kl_coeff": [0.1, 0.4],
        "vf_clip_param": [10.0, float("inf")],
        "grad_clip": [40, 200],
    },
    require_attrs=False
)

stopping_criteria = {"training_iteration": 500, "env_runners/episode_return_mean": 100}

config = (
    PPOConfig()
    .environment(
        PbEnvironment,
    )
    .env_runners(
        num_env_runners=31
    )
    .training(
        lr=tune.uniform(1e-5, 1e-3),
        gamma=tune.uniform(0.95, 0.99),
        lambda_=tune.uniform(0.97, 1.0),
        entropy_coeff=tune.choice([0.0, 0.01]),
        vf_loss_coeff=tune.uniform(0.01, 1.0),
        clip_param=tune.uniform(0.1, 0.3),
        kl_target=tune.uniform(0.01, 0.03),
        minibatch_size=tune.choice([512, 1024, 2048, 4096]),
        num_epochs=tune.randint(6, 32),
        vf_share_layers=tune.choice([True, False]),
        use_kl_loss=tune.choice([True, False]),
        kl_coeff=tune.uniform(0.1, 0.4),
        vf_clip_param=tune.choice([10.0, 40.0, float("inf")]),
        grad_clip=tune.choice([None, 40, 100, 200]),
        train_batch_size=tune.sample_from(
            lambda spec: spec.config["minibatch_size"] * 31
        ),
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
        scheduler=pb2_scheduler,
        num_samples=8,
    ),
    param_space=config,
    run_config=tune.RunConfig(stop=stopping_criteria),
)

# tuner = tune.Tuner.restore('C:/Users/andre/ray_results/PPO_2025-04-20_14-47-02', "PPO", resume_errored=True, restart_errored=True)

results = tuner.fit()

print(results)

best_result = results.get_best_result(metric="env_runners/episode_return_mean",mode="max")

print("Best performing trial's final set of hyperparameters:\n")
pprint.pprint(best_result.config)

print("\nBest performing trial's final reported metrics:\n")

metrics_to_print = [
    "episode_return_mean",
    "episode_reward_max",
    "episode_reward_min",
    "episode_len_mean",
]
pprint.pprint({k: v for k, v in best_result.metrics.items() if k in metrics_to_print})

print(best_result.checkpoint)

from ray.rllib.algorithms.algorithm import Algorithm

loaded_ppo = Algorithm.from_checkpoint(best_result.checkpoint)

torch.onnx.export(
    loaded_ppo.get_module(),
    {
        'batch': {
            'obs': torch.randn(1, 10)
        }
    },
    "models/movement.onnx",
)
