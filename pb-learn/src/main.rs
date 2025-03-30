mod env;
mod module;
mod ppo;

use burn::backend::wgpu::WgpuDevice;
use clap::Parser;
use env::Environment;
use ppo::PpoAgent;

#[derive(Parser)]
struct Args {
    #[clap(long, default_value = "2048")]
    horizon: u32,
    #[clap(long, default_value = "50000000")]
    max_steps: u32,
    #[clap(long, default_value = "0.99")]
    gamma: f32,
    #[clap(long, default_value = "0.95")]
    lambda: f32,
    #[clap(long, default_value = "0.2")]
    clip_rate: f32,
    #[clap(long, default_value = "10")]
    k_epochs: u32,
    #[clap(long, default_value = "150")]
    net_width: u32,
    #[clap(long, default_value = "0.0002")]
    actor_learning_rate: f32,
    #[clap(long, default_value = "0.0002")]
    critic_learning_rate: f32,
    #[clap(long, default_value = "0.001")]
    l2_regularization: f32,
    #[clap(long, default_value = "64")]
    actor_optimizer_batch_size: u32,
    #[clap(long, default_value = "64")]
    critic_optimizer_batch_size: u32,
    #[clap(long, default_value = "0.001")]
    entropy: f32,
    #[clap(long, default_value = "0.99")]
    entropy_decay: f32,
    #[clap(long)]
    seed: Option<u64>,
}

fn main() {
    let opts = Args::parse();

    let device = WgpuDevice::default();
    let mut env = Environment::new(opts.seed);

    let mut agent = PpoAgent::new(&opts);
    let mut traj_length = 0;

    for step in 0..opts.max_steps {
        let state = env.reset();
        let mut done = false;

        while !done {}
    }
}
