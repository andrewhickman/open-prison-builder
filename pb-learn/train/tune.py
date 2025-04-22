import pprint
from ray import tune
from ray.tune.schedulers import PopulationBasedTraining
from ray.rllib.algorithms.ppo.ppo import PPOConfig
from ray.rllib.core.rl_module.default_model_config import DefaultModelConfig
from ray.rllib.utils.metrics import NUM_ENV_STEPS_SAMPLED_LIFETIME

import torch

from env import PbEnvironment;

def explore(config):
    if config["train_batch_size"] < config["sgd_minibatch_size"] * 2:
        config["train_batch_size"] = config["sgd_minibatch_size"] * 2
    return config

pbt = PopulationBasedTraining(
    time_attr=f"{NUM_ENV_STEPS_SAMPLED_LIFETIME}",
    metric="env_runners/episode_return_mean",
    mode="max",
    perturbation_interval=50000,
    hyperparam_mutations={
        "lr": tune.uniform(1e-5, 1e-3),
        "gamma": tune.uniform(0.95, 0.99),
        "lambda_": tune.uniform(0.97, 1.0),
        "entropy_coeff": tune.choice([0.0, 0.01]),
        "vf_loss_coeff": tune.uniform(0.01, 1.0),
        "clip_param": tune.uniform(0.1, 0.3),
        "kl_target": tune.uniform(0.01, 0.03),
        "minibatch_size": tune.choice([512, 1024, 2048, 4096]),
        "num_epochs": tune.randint(6, 32),
        "vf_share_layers": tune.choice([True, False]),
        "use_kl_loss": tune.choice([True, False]),
        "kl_coeff": tune.uniform(0.1, 1.0),
        "vf_clip_param": tune.choice([10.0, 40.0, float("inf")]),
        "grad_clip": tune.choice([40, 100, 200]),
        "train_batch_size": tune.choice([16384, 32768, 65536, 131072]),
    },
    require_attrs=False
)

stopping_criteria = {"training_iteration": 500, "env_runners/episode_return_mean": 200}

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
        kl_coeff=tune.uniform(0.1, 1.0),
        vf_clip_param=tune.choice([10.0, 40.0, float("inf")]),
        grad_clip=tune.choice([None, 40, 100, 200]),
        train_batch_size=tune.choice([16384, 32768, 65536, 131072]),
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
        scheduler=pbt,
        num_samples=8,
    ),
    param_space=config,
    run_config=tune.RunConfig(stop=stopping_criteria),
)

# tuner = tune.Tuner.restore('C:/Users/andre/ray_results/PPO_2025-04-21_22-30-32', "PPO", resume_errored=True, restart_errored=True)

results = tuner.fit()

print(results)

best_result = results.get_best_result(metric="env_runners/episode_return_mean",mode="max")

print("Best performing trial's final set of hyperparameters:\n")
pprint.pprint(best_result.config)

print("\nBest performing trial's final reported metrics:\n")
pprint.pprint(best_result.metrics)

print(best_result.checkpoint)

from ray.rllib.algorithms.algorithm import Algorithm

loaded_ppo = Algorithm.from_checkpoint(best_result.checkpoint)

torch.onnx.export(
    loaded_ppo.get_module(),
    {
        'batch': {
            'obs': torch.randn(1, 12)
        }
    },
    "models/movement.onnx",
)
