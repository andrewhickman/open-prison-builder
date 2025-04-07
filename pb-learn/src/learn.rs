use std::{f32::consts::PI, io::Read, mem::replace};

use avian2d::{dynamics::integrator::IntegrationSet, prelude::*};
use base64::{engine::general_purpose::STANDARD, Engine};
use bevy::{ecs::system::SystemParam, prelude::*};
use blocking::unblock;
use flate2::bufread::GzDecoder;
use pb_assets::AssetHandles;
use pb_engine::pawn::{
    self, PawnBundle, MAX_ACCELERATION, MAX_ANGULAR_VELOCITY, MAX_TORQUE, MAX_VELOCITY,
};
use pb_util::callback::{spawn_compute, CallbackSender};
use prost::Message;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::Serialize;
use smallvec::SmallVec;
use tract_onnx::{onnx, pb::ModelProto, prelude::*};

use crate::rl_link::{Episode, EpisodesAndGetState, RlLinkClient, SetState};

pub struct PbLearnPlugin;

impl Plugin for PbLearnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RlLinkClient>()
            .init_resource::<LearnerState>();

        app.add_systems(Startup, |mut learner: Learner| learner.startup())
            .add_systems(
                FixedPreUpdate,
                (|mut learner: Learner| learner.pre_update()).run_if(have_model),
            )
            .add_systems(
                FixedUpdate,
                (|mut learner: Learner| learner.update()).run_if(have_model),
            )
            .add_systems(
                FixedPostUpdate,
                (|mut learner: Learner| learner.post_update())
                    .after(IntegrationSet::Position)
                    .run_if(have_model),
            );
    }
}

#[derive(Copy, Clone, Debug, Serialize)]
#[serde(into = "[f32; Self::SIZE]")]
pub struct Action {
    pub force: Vec2,
    pub torque: f32,
}

#[derive(Copy, Clone, Debug, Serialize)]
#[serde(into = "[f32; Self::SIZE]")]
pub struct Observation {
    pub rotation: Vec2,
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
    pub target: Vec2,
}

#[derive(Copy, Clone, Debug, Component)]
pub struct TargetPosition(pub Vec2);

#[derive(SystemParam)]
pub struct Learner<'w, 's> {
    commands: Commands<'w, 's>,
    client: Res<'w, RlLinkClient>,
    query: Query<
        'w,
        's,
        (
            &'static Position,
            &'static Rotation,
            &'static LinearVelocity,
            &'static AngularVelocity,
            &'static TargetPosition,
        ),
    >,
    state: ResMut<'w, LearnerState>,
    time: Res<'w, Time>,
    assets: Res<'w, AssetHandles>,
    sender: Res<'w, CallbackSender>,
}

#[derive(Resource)]
struct LearnerState {
    env_steps: usize,
    env_steps_per_sample: usize,
    model: Option<Model>,
    entities: Vec<Entity>,
    episodes: Vec<Episode<Observation, Action>>,
    rng: SmallRng,
}

type Model = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

impl Default for LearnerState {
    fn default() -> Self {
        LearnerState {
            env_steps: 0,
            env_steps_per_sample: 0,
            model: None,
            entities: Vec::new(),
            episodes: vec![Episode::default()],
            rng: SmallRng::from_os_rng(),
        }
    }
}

fn have_model(state: Res<LearnerState>) -> bool {
    state.model.is_some()
}

impl Learner<'_, '_> {
    fn startup(&mut self) {
        self.client.ping();
        info!("connected to RLlink");

        let config = self.client.get_config();
        self.state.env_steps_per_sample = config.env_steps_per_sample;

        let state = self.client.get_state();
        self.set_state(state);
    }

    fn pre_update(&mut self) {
        if self.state.entities.is_empty() {
            info!("reinitializing environment");
            self.reset();
        }
    }

    fn update(&mut self) {
        let observation = self.observe();
        self.episode_mut().obs.push(observation);

        let res = self
            .state
            .model
            .as_ref()
            .unwrap()
            .run(SmallVec::from_iter([TValue::from(observation.to_tensor())]))
            .unwrap();
        assert_eq!(res.len(), 1);
        let action_tensor = res.first().unwrap();
        let action = Action::from_tensor(action_tensor);
        self.act(action);

        self.episode_mut().actions.push(action);
    }

    fn post_update(&mut self) {
        let observation = self.observe();
        let prev_observation = self.episode().obs.last().unwrap();

        let reward = reward(prev_observation, &observation, &self.time);
        match reward {
            Some(reward) if observation.target.abs().max_element() > 10. => {
                self.episode_mut().rewards.push(reward);
                self.episode_mut().is_truncated = true;
                self.finish_episode(observation);
            }
            Some(reward) => {
                info!("reward: {reward}");
                self.episode_mut().rewards.push(reward);
            }
            None => {
                self.episode_mut().rewards.push(0.);
                self.episode_mut().is_terminated = true;
                self.finish_episode(observation);
            }
        }

        if self.state.env_steps == self.state.env_steps_per_sample {
            self.finish_episode(observation);
            return self.finish_step();
        }
        self.state.env_steps += 1;

        if self.state.env_steps % 100 == 0 {
            info!(
                "step {} out of {}, episode count {}, reward: {reward:?}",
                self.state.env_steps,
                self.state.env_steps_per_sample,
                self.state.episodes.len(),
            );
        }
    }

    fn finish_episode(&mut self, observation: Observation) {
        self.episode_mut().obs.push(observation);
        assert_eq!(self.episode().obs.len(), self.episode().actions.len() + 1);
        assert_eq!(self.episode().actions.len(), self.episode().rewards.len());
        self.state.episodes.push(Episode::default());
        for entity in self.state.entities.drain(..) {
            self.commands.entity(entity).despawn();
        }
    }

    fn reset(&mut self) {
        let mut position: Vec2 = self.state.rng.random::<[f32; 2]>().into();
        position *= 5.;
        let rotation = self.state.rng.random_range(-PI..PI);
        let mut target: Vec2 = self.state.rng.random::<[f32; 2]>().into();
        target *= 5.;

        let linear_velocity_angle = self.state.rng.random_range(-PI..PI);
        let max_velocity = MAX_VELOCITY.lerp(MAX_VELOCITY / 2., linear_velocity_angle.abs() / PI);
        let linear_velocity = Vec2::from_angle(rotation + linear_velocity_angle)
            * self.state.rng.random_range(0.0..max_velocity);

        let max_angular_velocity = MAX_ANGULAR_VELOCITY.lerp(
            MAX_ANGULAR_VELOCITY / 2.,
            linear_velocity.length() / MAX_VELOCITY,
        );
        let angular_velocity = self
            .state
            .rng
            .random_range(-max_angular_velocity..max_angular_velocity);

        let entity = self
            .commands
            .spawn(PawnBundle::new(Vec2::ZERO))
            .insert((
                Rotation::radians(rotation),
                LinearVelocity(linear_velocity),
                AngularVelocity(angular_velocity),
                TargetPosition(target),
            ))
            .id();
        self.state.entities.push(entity);

        let target = self
            .commands
            .spawn((
                Transform::from_translation(target.extend(0.)),
                Sprite {
                    custom_size: Some(Vec2::splat(pawn::RADIUS * 1.5)),
                    image: self.assets.close_icon.clone(),
                    ..default()
                },
            ))
            .id();
        self.state.entities.push(target);
    }

    fn observe(&self) -> Observation {
        let entity = self.state.entities[0];
        let (position, rotation, linear_velocity, angular_velocity, target) =
            self.query.get(entity).unwrap();
        Observation {
            rotation: Vec2::new(rotation.cos, rotation.sin),
            linear_velocity: linear_velocity.0,
            angular_velocity: angular_velocity.0,
            target: target.0 - position.0,
        }
    }

    fn act(&mut self, act: Action) {
        let entity = self.state.entities[0];
        self.commands.entity(entity).insert((
            ExternalForce::new(act.force.map(normalize) * MAX_ACCELERATION).with_persistence(false),
            ExternalTorque::new(normalize(act.torque) * MAX_TORQUE).with_persistence(false),
        ));
    }

    fn episode(&self) -> &Episode<Observation, Action> {
        self.state.episodes.last().unwrap()
    }

    fn episode_mut(&mut self) -> &mut Episode<Observation, Action> {
        self.state.episodes.last_mut().unwrap()
    }

    fn finish_step(&mut self) {
        let state = self.client.episodes_and_get_state(EpisodesAndGetState {
            episodes: replace(&mut self.state.episodes, vec![Episode::default()]),
            env_steps: self.state.env_steps,
        });

        self.state.model = None;
        self.set_state(state);
        self.state.env_steps = 0;
    }

    fn set_state(&mut self, state: SetState) {
        info!("weights_seq_no: {}", state.weights_seq_no);

        let sender = self.sender.clone();
        spawn_compute(async move {
            let model = unblock(move || parse_model(&state.onnx_file)).await;
            sender.send_oneshot_system_with_input(
                |In(model): In<Model>, mut learner: Learner| {
                    info!("finished parsing new model");
                    learner.state.model = Some(model);
                },
                model,
            );
        });
    }
}

impl Observation {
    const SIZE: usize = 7;

    pub fn to_tensor(self) -> Tensor {
        let array: [f32; Self::SIZE] = self.into();
        Tensor::from_shape(&[1, Self::SIZE], array.as_slice()).unwrap()
    }
}

impl Action {
    const SIZE: usize = 3;

    pub fn from_tensor(t: &Tensor) -> Self {
        let slice = t.as_slice().unwrap();
        Action {
            force: Vec2::new(slice[0], slice[1]),
            torque: slice[2],
        }
    }
}

impl From<Observation> for [f32; Observation::SIZE] {
    fn from(val: Observation) -> Self {
        [
            val.rotation.x,
            val.rotation.y,
            val.linear_velocity.x,
            val.linear_velocity.y,
            val.angular_velocity,
            val.target.x,
            val.target.y,
        ]
    }
}

impl From<Action> for [f32; Action::SIZE] {
    fn from(val: Action) -> Self {
        [val.force.x, val.force.y, val.torque]
    }
}

fn normalize(f: f32) -> f32 {
    if f.is_finite() {
        f
    } else {
        0.
    }
}

fn reward(prev: &Observation, curr: &Observation, time: &Time) -> Option<f32> {
    if curr.target.length() < 0.1 {
        return None;
    }
    // if relative_eq!(curr.target, Vec2::ZERO) {
    //     return None;
    // }

    let movement_reward =
        (prev.target.length() - curr.target.length()) / (MAX_VELOCITY * time.delta_secs());

    let angular_velocity_penalty = curr.angular_velocity.abs() / MAX_ANGULAR_VELOCITY;

    Some(movement_reward - angular_velocity_penalty)
}

fn parse_model(onnx_file: &str) -> Model {
    let onnx_bytes = STANDARD.decode(onnx_file).unwrap();
    let mut onnx_buf = Vec::new();
    GzDecoder::new(onnx_bytes.as_slice())
        .read_to_end(&mut onnx_buf)
        .unwrap();
    let model = ModelProto::decode(onnx_buf.as_slice()).unwrap();

    info!("parsing new model");
    onnx()
        .parse(&model, None)
        .unwrap()
        .model
        .into_optimized()
        .unwrap()
        .into_runnable()
        .unwrap()
}
