use bevy::{
    asset::{RenderAssetUsages, weak_handle},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_resource::{AsBindGroup, ShaderDefVal, ShaderRef},
    },
    sprite::{AlphaMode2d, Material2d},
};

use crate::{layer, projection::PIXELS_PER_METER};

pub const GRID_MESH_HANDLE: Handle<Mesh> = weak_handle!("ed4eaa8e-ce4f-4d43-abc6-aeb015e048f7");

const GRID_SHADER_HANDLE: Handle<Shader> = weak_handle!("deb5bce7-5b6e-4fbd-a268-9b829c3da570");

pub fn startup(mut meshes: ResMut<Assets<Mesh>>, mut shaders: ResMut<Assets<Shader>>) {
    meshes.insert(GRID_MESH_HANDLE.id(), mesh());
    shaders.insert(
        GRID_SHADER_HANDLE.id(),
        Shader::from_wgsl_with_defs(
            include_str!("../../assets/shaders/grid.wgsl"),
            "assets/shaders/grid.wgsl",
            vec![ShaderDefVal::UInt(
                "PIXELS_PER_METER".into(),
                PIXELS_PER_METER as _,
            )],
        ),
    );
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    #[cfg(not(target_arch = "wasm32"))]
    level: f32,
    #[uniform(1)]
    #[cfg(target_arch = "wasm32")]
    level: Vec4,
}

impl GridMaterial {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(color: LinearRgba) -> Self {
        GridMaterial { color, level: 1.0 }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(color: LinearRgba) -> Self {
        GridMaterial {
            color,
            level: Vec4::new(1., 0., 0., 0.),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_level(&mut self, level: f32) {
        self.level = level;
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_level(&mut self, level: f32) {
        self.level = Vec4::new(level, 0., 0., 0.);
    }
}

impl Material2d for GridMaterial {
    fn vertex_shader() -> ShaderRef {
        GRID_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        GRID_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

fn mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2]))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0, 0.0, layer::GRID]; 3])
}
