pub mod grid;

use bevy::{
    picking::backend::{ray::RayMap, HitData, PointerHits},
    prelude::*,
};
use grid::Grid;
use pb_engine::{root::Root, EngineState};
use pb_render::projection::ProjectionExt;
use pb_util::try_res_s;

#[derive(Event, Debug, Clone, Copy)]
pub struct SelectPoint {
    pub point: Vec2,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct CancelPoint;

#[derive(Event, Debug, Clone, Copy)]
pub struct ClickPoint {
    pub point: Vec2,
}

pub fn update_hits(
    ray_map: Res<RayMap>,
    camera_q: Query<(&Camera, &Projection)>,
    state: Res<State<EngineState>>,
    grid_q: Query<&Grid>,
    mut hits: EventWriter<PointerHits>,
) {
    let EngineState::Running(root) = *state.get() else {
        return;
    };

    for (&ray_id, &ray) in ray_map.iter() {
        let (camera, projection) = try_res_s!(camera_q.get(ray_id.camera));
        if !camera.is_active {
            continue;
        }

        let mut pos = ray.origin.xy();
        for grid in &grid_q {
            if let Some(mark) = grid.point_mark(ray.origin.xy(), projection.scale()) {
                pos = mark;
                break;
            }
        }

        let picks = vec![(
            root,
            HitData::new(ray_id.camera, 0., Some(pos.extend(0.)), None),
        )];
        hits.write(PointerHits::new(
            ray_id.pointer,
            picks,
            camera.order as f32 - 0.5,
        ));
    }
}

pub fn root_added(trigger: Trigger<OnAdd, Root>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(Pickable::default())
        .observe(over)
        .observe(moved)
        .observe(out)
        .observe(click);
}

fn over(mut trigger: Trigger<Pointer<Over>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(SelectPoint {
        point: trigger
            .event()
            .event
            .hit
            .position
            .expect("expected hit to have position")
            .xy(),
    });
}

fn moved(mut trigger: Trigger<Pointer<Move>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(SelectPoint {
        point: trigger
            .event()
            .event
            .hit
            .position
            .expect("expected hit to have position")
            .xy(),
    });
}

fn out(mut trigger: Trigger<Pointer<Out>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(CancelPoint);
}

fn click(mut trigger: Trigger<Pointer<Click>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(ClickPoint {
        point: trigger
            .event()
            .event
            .hit
            .position
            .expect("expected hit to have position")
            .xy(),
    });
}
