#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/settings.wgsl"::COLOUR_MULTIPLIER

@group(2) @binding(0) var<uniform> quad_colour: vec4<f32>;
@group(2) @binding(1) var height_map: texture_2d<f32>;
@group(2) @binding(2) var height_map_sampler: sampler;
@group(2) @binding(3) var colour_map: texture_2d<f32>;
@group(2) @binding(4) var colour_map_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
   // return quad_colour * textureSample(height_map, height_map_sampler, in.uv) * textureSample(colour_map, colour_map_sampler, in.uv) * COLOUR_MULTIPLIER;
   // return quad_colour * textureSample(height_map, height_map_sampler, in.uv) * COLOUR_MULTIPLIER;
   return quad_colour * textureSample(colour_map, colour_map_sampler, in.uv) * COLOUR_MULTIPLIER;
}
