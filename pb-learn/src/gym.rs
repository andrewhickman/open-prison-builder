

pub trait Environment {
    type Action;
    type Observation;

    fn action_space(&self) -> 

    fn step(&mut self, action: Self::Action) -> (Self::Observation, Step);

    fn reset(&mut self, seed: Option<u64>) -> Self::Observation;
}

pub struct Step {
    pub reward: f32,
    pub terminated: bool,
    pub truncated: bool,
}
