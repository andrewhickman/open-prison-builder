#import bevy_render::view::View

const PIXELS_PER_METER: f32 = #PIXELS_PER_METER;

@group(0) @binding(0) var<uniform> view: View;
@group(2) @binding(0) var<uniform> grid_color: vec4<f32>;
#ifdef SIXTEEN_BYTE_ALIGNMENT
@group(2) @binding(1) var<uniform> grid_level: vec4<f32>;
#else
@group(2) @binding(1) var<uniform> grid_level: f32;
#endif

struct VertexOutput {
    @builtin(position)
    position: vec4<f32>,
    @location(0)
    world_position: vec2<f32>,
    @location(1)
    scale: vec2<f32>,
};

@vertex
fn vertex(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Based on bevy's fullscreen_vertex_shader
    let uv = vec2<f32>(f32(vertex_index >> 1u), f32(vertex_index & 1u)) * 2.0;
    let clip_position = vec4<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), 0.0, 1.0);
    let world_position = (view.world_from_clip * clip_position).xy;
    let scale = (view.world_from_clip * vec4(1. / view.viewport.z, 1. / view.viewport.w, 0., 0.)).xy;

    return VertexOutput(clip_position, world_position, scale);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let level1 = mark_weight(in.world_position, grid_level * 1., 2., in.scale);
    let level2 = mark_weight(in.world_position, grid_level * 8., 4., in.scale);
    let level3 = mark_weight(in.world_position, grid_level * 64., 6., in.scale);
    let intensity = saturate(max(level1, max(level2, level3)));

    return vec4(grid_color.x, grid_color.y, grid_color.z, grid_color.a * intensity);
}

fn mark_weight(p: vec2<f32>, level: f32, width: f32, scale: vec2<f32>) -> f32 {
    let dist = smoothstep(scale * (width + 0.5), scale * (width - 0.5), distance_to_mark(p, level));

    let scale_min = .1 / PIXELS_PER_METER;
    let scale_max = 5. / PIXELS_PER_METER;
    let t = saturate((scale.x - scale_min) / (scale_max - scale_min));
    let fade = exp2(-12. * t / level);

    return max(dist.x, dist.y) * fade;
}

fn distance_to_mark(p: vec2<f32>, level: f32) -> vec2<f32> {
    return abs(round(p / level) * level - p);
}
