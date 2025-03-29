#![allow(unused)]

use std::f32::consts::FRAC_PI_4;

use candle::{DType, Device, Module, Tensor};
use candle_nn::{linear, seq, Activation, AdamW, Optimizer, ParamsAdamW, VarBuilder, VarMap};
use glam::Vec2;
use env::{Action, Environment, Observation};

mod env;

fn main() {
    let device = Device::Cpu;

    let varmap = VarMap::new();
    let var_builder = VarBuilder::from_varmap(&varmap, DType::F32, &device);

    let model = seq()
        .add(linear(Observation::SIZE, 16, var_builder.pp("lin1")).unwrap())
        .add(Activation::Relu)
        .add(linear(16, Action::SIZE, var_builder.pp("lin2")).unwrap());

    let mut env = Environment::new();
    let mut observations = Vec::new();

    let optimizer_params = ParamsAdamW {
        lr: 0.01,
        weight_decay: 0.01,
        ..Default::default()
    };

    let mut optimizer = AdamW::new(varmap.all_vars(), optimizer_params).unwrap();

    for epoch_idx in 0..100 {
        env.reset();
        let mut prev_observations = env.observe();
        let mut rewards: Vec<Option<f32>> = Vec::new();

        loop {
            let actions = prev_observations.iter().map(|(entity, observation)| {
                (*entity, Action::from_tensor(&model.forward(&observation.to_tensor(&device)).unwrap()))
            }).collect();

            let observations = env.step(&actions);
            let reward = env::all_rewards(&prev_observations, &observations);

            rewards.push(reward);
            prev_observations = match reward {
                Some(reward) => observations,
                None => {
                    if rewards.len() > 5000 {
                        break;
                    } else {
                        env.reset()
                    }
                },
            }
        }

        let total_reward: f32 = rewards.iter().flatten().sum();
        let episode_count = rewards.iter().filter(|r| r.is_none()).count();
        println!(
            "epoch: {:<3} episodes: {:<5} avg reward per episode: {:.2}",
            epoch_idx,
            episode_count,
            total_reward / episode_count as f32
        );

        let batch_size = rewards.len();

        let rewards = Tensor::from_vec(accumulate_rewards(&rewards), batch_size, &Device::Cpu).unwrap();

        let actions_mask = {
            let actions: Vec<i64> = steps.iter().map(|s| s.action).collect();
            let actions_mask: Vec<Tensor> = actions
                .iter()
                .map(|&action| {
                    // One-hot encoding
                    let mut action_mask = vec![0.0; env.action_space()];
                    action_mask[action as usize] = 1.0;

                    Tensor::from_vec(action_mask, env.action_space(), &Device::Cpu)
                        .unwrap()
                        .to_dtype(DType::F32)
                        .unwrap()
                })
                .collect();
            Tensor::stack(&actions_mask, 0)?.detach()
        };

        let states = {
            let states: Vec<Tensor> = steps.into_iter().map(|s| s.state).collect();
            Tensor::stack(&states, 0)?.detach()
        };

        let log_probs = actions_mask
            .mul(&log_softmax(&model.forward(&states)?, 1)?)?
            .sum(1)?;

        let loss = rewards.mul(&log_probs)?.neg()?.mean_all()?;
        optimizer.backward_step(&loss)?;
    }
}

fn accumulate_rewards(rewards: &[Option<f32>]) -> Vec<f32> {
    let mut acc_rewards: Vec<f32> = vec![0.; rewards.len()];
    let mut acc_reward = 0.;
    for (i, &reward) in rewards.iter().enumerate().rev() {
        match reward{
            Some(reward) => acc_reward += reward,
            None => acc_reward = 0.,
        }

        acc_rewards[i] = acc_reward;
    }

    acc_rewards
}
