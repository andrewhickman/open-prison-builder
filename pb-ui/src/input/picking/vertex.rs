use bevy::{picking::backend::*, prelude::*};
use pb_engine::EngineState;
use ray::RayMap;

use crate::input::InputState;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VertexPickingState {
    None,
    #[expect(unused)]
    Select,
    #[expect(unused)]
    Create,
}

impl ComputedStates for VertexPickingState {
    type SourceStates = InputState;

    fn compute(source: Self::SourceStates) -> Option<Self> {
        match source {
            InputState::Default => Some(VertexPickingState::None),
        }
    }
}

pub fn backend(
    ray_map: Res<RayMap>,
    camera_q: Query<&Camera>,
    state: Res<State<EngineState>>,
    mut hits: EventWriter<PointerHits>,
) {
    let EngineState::Running(root) = *state.get() else {
        error!("fail 1");
        return;
    };

    for (&ray_id, &ray) in ray_map.iter() {
        if let Ok(camera) = camera_q.get(ray_id.camera) {
            let picks = vec![(
                root,
                HitData::new(ray_id.camera, 0.0, Some(ray.origin.with_z(0.0)), None),
            )];
            hits.send(PointerHits::new(
                ray_id.pointer,
                picks,
                camera.order as f32 - 0.5,
            ));
        } else {
            error!("fail 2");
        }
    }
}

pub fn dbg_hits(mut hits: EventReader<PointerHits>) {
    for hit in hits.read() {
        if !hit.picks.is_empty() && hit.order < 0.5 {
            info!("{hit:?}");
        }
    }
}
