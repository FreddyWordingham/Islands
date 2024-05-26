#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/settings.wgsl"::COLOUR_MULTIPLIER

@group(2) @binding(0) var<uniform> quad_colour: vec4<f32>;
@group(2) @binding(1) var quad_texture: texture_2d<f32>;
@group(2) @binding(2) var base_colour_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
   let whiteness = textureSample(quad_texture, base_colour_sampler, in.uv);
   let height = whiteness.r;

   var colour = vec4<f32>(1.0, 0.0, 1.0, 1.0);
   if height < 0.1 {
      colour = vec4<f32>(0.0, 0.0, 1.0, 1.0);
   } else if height < 0.2 {
      colour = vec4<f32>(0.0, 1.0, 0.0, 1.0);
   } else if height < 0.3 {
      colour = vec4<f32>(1.0, 1.0, 0.0, 1.0);
   } else if height < 0.4 {
      colour = vec4<f32>(1.0, 0.0, 0.0, 1.0);
   }

   return quad_colour * whiteness * colour * COLOUR_MULTIPLIER;
}
