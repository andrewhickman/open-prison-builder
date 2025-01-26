use bevy::{
    picking::backend::{ray::RayMap, HitData, PointerHits},
    prelude::*,
};
use pb_engine::{EngineState, Root};

use super::PickingState;

pub fn backend(
    ray_map: Res<RayMap>,
    camera_q: Query<&Camera>,
    state: Res<State<EngineState>>,
    mut hits: EventWriter<PointerHits>,
) {
    let EngineState::Running(root) = *state.get() else {
        return;
    };

    for (&ray_id, &ray) in ray_map.iter() {
        if let Ok(camera) = camera_q.get(ray_id.camera) {
            let picks = vec![(
                root,
                HitData::new(ray_id.camera, 0., Some(ray.origin.with_z(0.)), None),
            )];
            hits.send(PointerHits::new(
                ray_id.pointer,
                picks,
                camera.order as f32 - 0.5,
            ));
        }
    }
}

pub fn root_added(trigger: Trigger<OnAdd, Root>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(PickingBehavior::default())
        .observe(over)
        .observe(moved)
        .observe(out)
        .observe(click);
}

pub fn over(
    mut trigger: Trigger<Pointer<Over>>,
    mut state: ResMut<PickingState>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    state.vertex_over(&mut commands, trigger.event());
}

pub fn moved(
    mut trigger: Trigger<Pointer<Move>>,
    mut state: ResMut<PickingState>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    state.vertex_move(&mut commands, trigger.event());
}

pub fn out(
    mut trigger: Trigger<Pointer<Out>>,
    mut state: ResMut<PickingState>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    state.vertex_out(&mut commands, trigger.event());
}

pub fn click(
    mut trigger: Trigger<Pointer<Click>>,
    mut state: ResMut<PickingState>,
    engine_state: Res<State<EngineState>>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    state.vertex_click(&mut commands, &engine_state, trigger.event());
}
