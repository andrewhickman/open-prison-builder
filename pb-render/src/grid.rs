use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::Material2d,
};
use pb_util::weak_handle;

pub const GRID_MESH_HANDLE: Handle<Mesh> = weak_handle!("ed4eaa8e-ce4f-4d43-abc6-aeb015e048f7");

pub fn startup(mut meshes: ResMut<Assets<Mesh>>) {
    meshes.insert(GRID_MESH_HANDLE.id(), mesh());
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material2d for GridMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/grid.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/grid.wgsl".into()
    }
}

fn mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2]))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0, 0.0, 0.0]; 3])
}
