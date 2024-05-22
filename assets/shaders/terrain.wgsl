#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/settings.wgsl"::COLOUR_MULTIPLIER

@group(2) @binding(0) var<uniform> quad_colour: vec4<f32>;
@group(2) @binding(1) var quad_texture: texture_2d<f32>;
@group(2) @binding(2) var base_colour_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
   return quad_colour * textureSample(quad_texture, base_colour_sampler, in.uv) * COLOUR_MULTIPLIER;
}
