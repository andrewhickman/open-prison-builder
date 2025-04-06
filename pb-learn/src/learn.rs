use std::io::Read;

use base64::{engine::general_purpose::STANDARD, Engine};
use bevy::prelude::*;
use candle_onnx::onnx::ModelProto;
use flate2::bufread::GzDecoder;
use prost::Message;

use crate::rl_link::RlLinkClient;

pub struct PbLearnPlugin;

impl Plugin for PbLearnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RlLinkClient>();

        app.add_systems(Startup, startup)
            .add_systems(FixedUpdate, update);
    }
}

#[derive(Resource)]
struct State {
    remaining_steps: u32,
    model: ModelProto,
}

pub fn startup(mut commands: Commands, client: Res<RlLinkClient>) {
    client.ping();
    info!("connected to RLlink");

    let config = dbg!(client.get_config());

    let state = client.get_state();
    let onnx = STANDARD.decode(state.onnx_file).unwrap();
    let mut onnx_buf = Vec::new();
    GzDecoder::new(onnx.as_slice()).read_to_end(&mut onnx_buf).unwrap();

    let model = ModelProto::decode(onnx_buf.as_slice()).unwrap();

    commands.insert_resource(State {
        remaining_steps: config.env_steps_per_sample,
        model,
    });
}

pub fn update(client: Res<RlLinkClient>, state: ResMut<State>) {

}