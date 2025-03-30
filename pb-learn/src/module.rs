use burn::{
    nn::{Linear, LinearConfig},
    prelude::*, tensor::activation::{sigmoid, softplus},
};

#[derive(Module, Debug)]
pub struct GaussianActor<B: Backend> {
    l1: Linear<B>,
    l2: Linear<B>,
    mu_head: Linear<B>,
    sigma_head: Linear<B>,
}

impl<B: Backend> GaussianActor<B> {
    pub fn new(device: &Device<B>, state_dim: usize, action_dim: usize, net_width: usize) -> Self {
        GaussianActor {
            l1: LinearConfig::new(state_dim, net_width).init(device),
            l2: LinearConfig::new(net_width, net_width).init(device),
            mu_head: LinearConfig::new(net_width, action_dim).init(device),
            sigma_head: LinearConfig::new(net_width, action_dim).init(device),
        }
    }

    pub fn forward<const D: usize>(&self, state: Tensor<B, D>) -> (Tensor<B, D>, Tensor<B, D>) {
        let a = self.l1.forward(state).tanh();
        let a = self.l2.forward(a).tanh();

        let mu = sigmoid(self.mu_head.forward(a.clone()));
        let sigma = softplus(self.sigma_head.forward(a), 1.0);

        (mu, sigma)
    }

    pub fn get_dist<const D: usize>(&self, state: Tensor<B, D>) {
        let (alpha, beta) = self.forward(state);
        let dist = Beta
    }
}
