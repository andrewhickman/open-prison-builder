use bevy::{color::palettes::tailwind::GREEN_300, prelude::*};

use crate::map::mesh::MapMesh;

#[derive(Default, Resource)]
pub struct DevSettings {
    pub draw_paths: bool,
    pub draw_meshes: bool,
}

pub fn draw_paths_condition(settings: Res<DevSettings>) -> bool {
    settings.draw_paths
}

pub fn draw_paths() {}

// pub fn draw_paths(task_q: Query<(&Task, &PathTask)>, pos_q: Query<&Position>, mut gizmos: Gizmos) {
//     for (task, path) in &task_q {
//         if let Some(steps) = path.steps() {
//             if let Ok(start) = pos_q.get(task.actor()) {
//                 if !steps.is_empty() {
//                     gizmos.line_2d(start.0, steps[0], INDIGO_800);
//                     for i in 0..(steps.len() - 1) {
//                         gizmos.line_2d(steps[i], steps[i + 1], INDIGO_800);
//                     }
//                 }
//             }
//         }
//     }
// }

pub fn draw_meshes_condition(settings: Res<DevSettings>) -> bool {
    settings.draw_meshes
}

pub fn draw_meshes(map_q: Query<&MapMesh>, mut gizmos: Gizmos) {
    for map in &map_q {
        for mesh in map.meshes() {
            for layer in &mesh.layers {
                for polygon in &layer.polygons {
                    gizmos.linestrip(
                        polygon
                            .vertices
                            .iter()
                            .cycle()
                            .take(polygon.vertices.len() + 1)
                            .map(|&index| layer.vertices[index as usize].coords.extend(0.)),
                        GREEN_300,
                    );
                }
            }
        }
    }
}
