#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> color: vec4<f32>;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;

struct FragmentOutput {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn fragment(mesh: VertexOutput) -> FragmentOutput {
    let color = color * textureSample(texture, texture_sampler, mesh.uv);
    return FragmentOutput(color, 1. - mesh.uv.y);
}
