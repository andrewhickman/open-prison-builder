pub mod pawn;
pub mod wall;

use avian2d::{
    math::{AdjustPrecision, Vector},
    prelude::*,
};
use bevy::{
    math::FloatOrd,
    picking::backend::{HitData, PointerHits, ray::RayMap},
    prelude::*,
};
use pb_engine::picking::Layer;
use pb_render::projection::ProjectionExt;
use pb_util::try_res_s;

use super::PHYSICS_PICKING_THRESHOLD;

#[derive(Default, Clone, Copy, Debug, Component)]
pub enum PhysicsPickingState {
    #[default]
    Default,
    Wall,
}

/// Queries for collider intersections with pointers using [`PhysicsPickingSettings`] and sends [`PointerHits`] events.
pub fn update_hits(
    camera_q: Query<(&Camera, &Projection)>,
    ray_map: Res<RayMap>,
    pickable_q: Query<&Pickable>,
    spatial_query: SpatialQuery,
    state: Option<Single<&PhysicsPickingState>>,
    mut output_events: EventWriter<PointerHits>,
) {
    let state = state.map(|s| s.to_owned()).unwrap_or_default();

    for (&ray_id, &ray) in ray_map.iter() {
        let (camera, projection) = try_res_s!(camera_q.get(ray_id.camera));
        if !camera.is_active {
            continue;
        }

        let mut hits: Vec<(Entity, HitData)> = vec![];
        state.execute(
            &spatial_query,
            ray.origin.truncate().adjust_precision(),
            projection.scale(),
            |entity| {
                let is_pickable = pickable_q
                    .get(entity)
                    .map(|p| p.is_hoverable)
                    .unwrap_or(true);

                if is_pickable {
                    hits.push((
                        entity,
                        HitData::new(ray_id.camera, 0.0, Some(ray.origin), None),
                    ));
                }

                true
            },
        );

        hits.sort_unstable_by_key(|hit| {
            FloatOrd(
                hit.1
                    .position
                    .unwrap()
                    .xy()
                    .distance_squared(ray.origin.xy()),
            )
        });
        hits.truncate(1);

        output_events.write(PointerHits::new(ray_id.pointer, hits, camera.order as f32));
    }
}

impl PhysicsPickingState {
    fn execute<F>(&self, query: &SpatialQuery, position: Vector, scale: f32, callback: F)
    where
        F: FnMut(Entity) -> bool,
    {
        match self {
            PhysicsPickingState::Default => {
                query.point_intersections_callback(position, &SpatialQueryFilter::DEFAULT, callback)
            }
            PhysicsPickingState::Wall => query.shape_intersections_callback(
                &Collider::circle(PHYSICS_PICKING_THRESHOLD * scale),
                position,
                0.,
                &SpatialQueryFilter::from_mask(Layer::Wall),
                callback,
            ),
        }
    }
}
