#![allow(clippy::single_range_in_vec_init)]

mod env;

use core::f32;
use std::f32::consts::TAU;

use burn::{
    backend::{wgpu::WgpuDevice, Autodiff, Wgpu},
    grad_clipping::GradientClipping,
    nn::{Initializer, Linear, LinearConfig},
    optim::{AdamWConfig, GradientsParams, Optimizer},
    prelude::*,
    tensor::{
        activation::{relu, tanh},
        Distribution,
    },
};
use env::{Action, Environment, Observation};
use rand::rng;
use rand_distr::{Distribution as _, Normal};

const NSTEPS: usize = 256;
const UPDATES: usize = 1000000;
const OPTIM_BATCHSIZE: usize = 64;
const OPTIM_EPOCHS: usize = 4;

#[derive(Module, Debug)]
struct PpoModel<B: Backend> {
    input: Linear<B>,
    actor_mu: Linear<B>,
    actor_sigma: Linear<B>,
    critic: Linear<B>,
}

impl<B: Backend> PpoModel<B> {
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let initializer = Initializer::XavierUniform { gain: 1.0 };
        Self {
            input: LinearConfig::new(input_size, hidden_size)
                .with_initializer(initializer.clone())
                .init(&Default::default()),
            actor_mu: LinearConfig::new(hidden_size, output_size)
                .with_initializer(initializer.clone())
                .init(&Default::default()),
            actor_sigma: LinearConfig::new(hidden_size, output_size)
                .with_initializer(initializer.clone())
                .init(&Default::default()),
            critic: LinearConfig::new(hidden_size, 1)
                .with_initializer(initializer)
                .init(&Default::default()),
        }
    }

    fn forward(&self, input: Tensor<B, 2>) -> (Tensor<B, 2>, Tensor<B, 2>, Tensor<B, 2>) {
        let output = relu(self.input.forward(input));
        let mu = tanh(self.actor_mu.forward(output.clone()));
        let sigma = tanh(self.actor_sigma.forward(output.clone()));
        let values = self.critic.forward(output);

        (values, mu, sigma)
    }
}

pub fn main() {
    let mut env = Environment::new(None);
    println!("action space: {}", env.action_space());
    println!("observation space: {:?}", env.observation_space());

    let device = WgpuDevice::default();
    let mut model =
        PpoModel::<Autodiff<Wgpu>>::new(env.observation_space()[0], 5, env.action_space());
    let mut opt = AdamWConfig::new()
        .init()
        .with_grad_clipping(GradientClipping::Norm(0.5));

    let mut state = Observation::to_tensor(&env.reset(), &device);

    let mut sum_rewards = Tensor::zeros([env.entity_count()], &device);
    let mut total_rewards = 0f64;
    let mut total_episodes = 0f64;

    let train_size = NSTEPS * env.entity_count();
    for update_index in 0..UPDATES {
        let mut s_states =
            Tensor::<_, 3>::zeros([NSTEPS, env.entity_count(), Observation::SIZE], &device);
        let mut s_values = Tensor::<_, 2>::zeros([NSTEPS, env.entity_count()], &device);
        let mut s_rewards = Tensor::<_, 2>::zeros([NSTEPS, env.entity_count()], &device);
        let mut s_actions =
            Tensor::<_, 3>::zeros([NSTEPS, env.entity_count(), Action::SIZE], &device);
        let mut s_masks = Tensor::<_, 2>::zeros([NSTEPS, env.entity_count()], &device);

        for s in 0..NSTEPS {
            let (critic, actor_mu, actor_sigma) = model.clone().no_grad().forward(state);
            let actions = tanh(sample_normal(actor_mu, actor_sigma, &device));
            let step = env.step(&device, &Action::from_tensor(&actions.clone()));

            sum_rewards = sum_rewards + step.reward.clone();
            total_rewards += (sum_rewards.clone() * step.done.clone().float())
                .sum()
                .into_data()
                .as_slice::<f32>()
                .unwrap()[0] as f64;
            total_episodes += step
                .done
                .clone()
                .float()
                .sum()
                .into_data()
                .as_slice::<f32>()
                .unwrap()[0] as f64;

            let masks = step.done.bool_not().float();
            sum_rewards = sum_rewards.mul(masks.clone());
            s_states = s_states.slice_assign(
                [(s..(s + 1)), 0..env.entity_count(), (0..Observation::SIZE)],
                step.state.clone().unsqueeze(),
            );
            s_actions = s_actions.slice_assign(
                [s..(s + 1), 0..env.entity_count(), (0..Action::SIZE)],
                actions.unsqueeze(),
            );
            s_values = s_values.slice_assign([s..(s + 1), 0..env.entity_count()], critic);
            s_rewards = s_rewards
                .slice_assign([s..(s + 1), 0..env.entity_count()], step.reward.unsqueeze());
            s_masks = s_masks.slice_assign([s..(s + 1), 0..env.entity_count()], masks.unsqueeze());

            state = step.state;
        }
        let states = s_states
            .clone()
            .narrow(0, 0, NSTEPS)
            .reshape([train_size, Observation::SIZE]);
        let returns = {
            let mut r = Tensor::<_, 2>::zeros([NSTEPS + 1, env.entity_count()], &device);
            let critic = model
                .clone()
                .no_grad()
                .forward(
                    s_states
                        .slice([((NSTEPS - 1) as _, NSTEPS as _)])
                        .squeeze(0),
                )
                .0;
            r = r.slice_assign(
                [NSTEPS - 1..NSTEPS],
                critic.reshape([env.entity_count()]).unsqueeze(),
            );
            for s in (0..NSTEPS).rev() {
                let r_s = s_rewards.clone().slice([s..(s + 1)])
                    + r.clone().slice([(s + 1)..(s + 2)])
                        * s_masks.clone().slice([s..(s + 1)])
                        * 0.99;
                r = r.slice_assign([s..(s + 1)], r_s);
            }
            r.narrow(0, 0, NSTEPS).reshape([train_size, 1])
        };
        let actions = s_actions.clone().reshape([train_size, Action::SIZE]);
        for _ in 0..OPTIM_EPOCHS {
            let batch_indexes = Tensor::random(
                [OPTIM_BATCHSIZE],
                Distribution::Uniform(0., (train_size + 1) as f64),
                &device,
            );
            let states = states.clone().select(0, batch_indexes.clone());
            let actions = actions.clone().select(0, batch_indexes.clone());
            let returns = returns.clone().select(0, batch_indexes);
            let (critic, actor_mu, actor_sigma) = model.forward(states);
            let action_log_probs = log_prob(actor_mu, actor_sigma.clone(), actions);
            let dist_entropy = entropy(actor_sigma);
            let advantages = returns.clone() - critic;
            let value_loss = advantages.clone().powi_scalar(2).mean_dim(1);
            let action_loss = (-advantages.detach() * action_log_probs).mean_dim(1);
            let loss = value_loss * 0.5 + action_loss - dist_entropy * 0.01;

            let grads = GradientsParams::from_grads(loss.backward(), &model);
            model = opt.step(0.001, model, grads);
        }
        // if update_index > 0 && update_index % 25 == 0 {
        println!(
            "{} {:.0} {}",
            update_index,
            total_episodes,
            total_rewards / total_episodes
        );
        total_rewards = 0.;
        total_episodes = 0.;
        // }
    }
}

fn sample_normal<B: Backend>(
    mu: Tensor<B, 2>,
    sigma: Tensor<B, 2>,
    device: &Device<B>,
) -> Tensor<B, 2> {
    let mu_data = mu.to_data();
    let sigma_data = sigma.to_data();
    assert_eq!(mu_data.num_elements(), sigma_data.num_elements());

    let data = Iterator::zip(
        mu.to_data().as_slice::<f32>().unwrap().iter(),
        sigma.to_data().as_slice::<f32>().unwrap().iter(),
    )
    .map(|(&mu, &sigma)| match Normal::new(mu, sigma) {
        Ok(distr) => distr.sample(&mut rng()).tanh(),
        Err(_) => f32::NAN,
    })
    .collect::<Vec<f32>>();

    Tensor::from_floats(TensorData::new(data, mu.shape()), device)
}

fn log_prob<B: Backend>(
    mu: Tensor<B, 2>,
    sigma: Tensor<B, 2>,
    value: Tensor<B, 2>,
) -> Tensor<B, 2> {
    let var = sigma.clone().powi_scalar(2);
    let log_scale = sigma.log();

    -((value - mu).powi_scalar(2) / (var * 2)) - log_scale - TAU.sqrt().ln()
}

fn entropy<B: Backend>(sigma: Tensor<B, 2>) -> Tensor<B, 2> {
    sigma.log() * TAU.ln() * 0.5 + 0.5
}
