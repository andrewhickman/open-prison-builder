pub mod wall;

use avian2d::prelude::PhysicsPickingFilter;
use bevy::prelude::*;
use pb_engine::picking::Layer;
use pb_util::try_res_s;

#[derive(Default, Clone, Copy, Debug, Component)]
pub enum ColliderPickingState {
    #[default]
    Default,
    Wall,
}

pub fn filter_added(
    trigger: Trigger<OnAdd, ColliderPickingState>,
    state_q: Query<&ColliderPickingState>,
    mut filter_q: Query<&mut PhysicsPickingFilter>,
) {
    let state = try_res_s!(state_q.get(trigger.entity()));

    for mut filter in &mut filter_q {
        *filter = match state {
            ColliderPickingState::Default => PhysicsPickingFilter::default(),
            ColliderPickingState::Wall => PhysicsPickingFilter::from_mask(Layer::Wall),
        }
    }
}

pub fn filter_removed(
    _: Trigger<OnAdd, ColliderPickingState>,
    mut filter_q: Query<&mut PhysicsPickingFilter>,
) {
    for mut filter in &mut filter_q {
        *filter = PhysicsPickingFilter::default();
    }
}
