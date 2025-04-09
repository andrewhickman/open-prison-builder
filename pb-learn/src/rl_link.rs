use core::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct RlLinkClient {
    stream: TcpStream,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(bound(
    serialize = "Episode<O, A>: Serialize",
    deserialize = "Episode<O, A>: Deserialize<'de>"
))]
enum Message<const O: usize = 0, const A: usize = 0> {
    Ping,
    Pong,
    GetConfig,
    SetConfig(SetConfig),
    GetState,
    EpisodesAndGetState(EpisodesAndGetState<O, A>),
    SetState(SetState),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetConfig {
    pub env_steps_per_sample: usize,
    pub force_on_policy: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound(
    serialize = "Episode<O, A>: Serialize",
    deserialize = "Episode<O, A>: Deserialize<'de>"
))]
pub struct EpisodesAndGetState<const O: usize, const A: usize> {
    pub episodes: Vec<Episode<O, A>>,
    pub env_steps: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetState {
    pub weights_seq_no: u32,
    pub onnx_file: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound(
    serialize = "[f32; O]: Serialize, [f32; A]: Serialize, [[f32; 2]; A]: Serialize",
    deserialize = "[f32; O]: Deserialize<'de>, [f32; A]: Deserialize<'de>, [[f32; 2]; A]: Deserialize<'de>",
))]
pub struct Episode<const O: usize, const A: usize> {
    pub obs: Vec<[f32; O]>,
    pub actions: Vec<[f32; A]>,
    pub action_dist_inputs: Vec<Vec<f32>>,
    pub action_logp: Vec<f32>,
    pub rewards: Vec<f32>,
    pub is_terminated: bool,
    pub is_truncated: bool,
}

impl RlLinkClient {
    pub fn new() -> Self {
        let stream = TcpStream::connect("localhost:27308").unwrap();
        // stream.set_read_timeout(Some(Duration::from_secs(30))).unwrap();
        RlLinkClient { stream }
    }

    pub fn ping(&self) {
        match self.request::<0, 0>(&Message::Ping) {
            Message::Pong => (),
            res => panic!("unexpected response type for PING {res:?}"),
        }
    }

    pub fn get_config(&self) -> SetConfig {
        match self.request::<0, 0>(&Message::GetConfig) {
            Message::SetConfig(res) => res,
            res => panic!("unexpected response type for GET_CONFIG {res:?}"),
        }
    }

    pub fn get_state(&self) -> SetState {
        match self.request::<0, 0>(&Message::GetState) {
            Message::SetState(res) => res,
            res => panic!("unexpected response type for GET_CONFIG {res:?}"),
        }
    }

    pub fn episodes_and_get_state<const O: usize, const A: usize>(
        &self,
        request: EpisodesAndGetState<O, A>,
    ) -> SetState
    where
        Episode<O, A>: Serialize,
    {
        match self.request(&Message::EpisodesAndGetState(request)) {
            Message::SetState(res) => res,
            res => panic!("unexpected response type for EPISODES_AND_GET_STATE {res:?}"),
        }
    }

    fn request<const O: usize, const A: usize>(&self, request: &Message<O, A>) -> Message
    where
        Episode<O, A>: Serialize,
    {
        let mut buf = Vec::with_capacity(24);
        buf.extend_from_slice(&[0; 8]);
        serde_json::to_writer(&mut buf, request).unwrap();
        let len = buf.len() - 8;
        write!(&mut buf[..8], "{:08}", len).unwrap();
        (&self.stream).write_all(&buf).unwrap();

        buf.truncate(8);
        (&self.stream).read_exact(&mut buf[..8]).unwrap();
        let len: usize = str::from_utf8(&buf).unwrap().parse().unwrap();
        buf.resize(len, 0);
        (&self.stream).read_exact(&mut buf).unwrap();
        serde_json::from_slice(&buf).unwrap()
    }
}

impl Default for RlLinkClient {
    fn default() -> Self {
        RlLinkClient::new()
    }
}

impl<const O: usize, const A: usize> Default for Episode<O, A> {
    fn default() -> Self {
        Self {
            obs: Default::default(),
            actions: Default::default(),
            rewards: Default::default(),
            is_terminated: Default::default(),
            is_truncated: Default::default(),
            action_dist_inputs: Default::default(),
            action_logp: Default::default(),
        }
    }
}
