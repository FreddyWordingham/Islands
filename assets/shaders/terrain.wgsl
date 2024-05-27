#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/settings.wgsl"::COLOUR_MULTIPLIER

@group(2) @binding(0) var<uniform> mouse_position: vec2<f32>;
@group(2) @binding(1) var<uniform> quad_colour: vec4<f32>;
@group(2) @binding(2) var height_map: texture_2d<f32>;
@group(2) @binding(3) var height_map_sampler: sampler;
@group(2) @binding(4) var colour_map: texture_2d<f32>;
@group(2) @binding(5) var colour_map_sampler: sampler;


fn blocked_line_of_sight(start: vec2<i32>, end: vec2<i32>, texture_dimensions: vec2<u32>) -> f32 {
    let fx = 1.0 / f32(texture_dimensions.x);
    let fy = 1.0 / f32(texture_dimensions.y);

    var x0: i32 = start.x;
    var y0: i32 = start.y;
    var h0: f32 = textureSample(height_map, height_map_sampler, vec2<f32>(f32(x0) * fx, f32(y0) * fy)).x;
    let x1: i32 = end.x;
    let y1: i32 = end.y;
    // let h1: f32 = textureSample(height_map, height_map_sampler, vec2<f32>(f32(x1) * fx, f32(y1) * fy)).x;
    let h1: f32 = 1.2;

    let dh: f32 = (h1 - h0) / sqrt(f32((abs(x1 - x0) * abs(x1 - x0)) + (abs(y1 - y0) * abs(y1 - y0))));

    var dx: i32 = abs(x1 - x0);
    var sx = -1;
    if x0 < x1 {
        sx = 1;
    }

    var dy: i32 = -abs(y1 - y0);
    var sy = -1;
    if y0 < y1 {
        sy = 1;
    }
    var error: i32 = dx + dy;

    loop {
        let ray_height = h0 + (dh * sqrt(f32((abs(x0 - start.x) * abs(x0 - start.x)) + (abs(y0 - start.y) * abs(y0 - start.y)))));

        let position = vec2<f32>(f32(x0) * fx, f32(y0) * fy);
        let height = textureSample(height_map, height_map_sampler, position);
        if height.x > (ray_height + 0.01) {
            return 0.9;
        }

        if (x0 == x1 && y0 == y1) {
            break;
        }

        let e2: i32 = 2 * error;

        if e2 >= dy {
            if x0 == x1 {
                break;
            }
            error += dy;
            x0 += sx;
        }

        if e2 <= dx {
            if y0 == y1 {
                break;
            }
            error += dx;
            y0 += sy;
        }
    }

    return 0.0;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var colour = COLOUR_MULTIPLIER;

    let texture_dimensions = textureDimensions(height_map).xy;
    let sun_position_x = f32(texture_dimensions.x) * mouse_position.x;
    let sun_position_y = f32(texture_dimensions.y) * mouse_position.y;
    let sun_position_xi = i32(sun_position_x);
    let sun_position_yi = i32(sun_position_x);

    let distance = distance(mouse_position, in.uv);
    let sun_radius = 0.1;
    if (distance < sun_radius) {
        colour = vec4<f32>(1.0, 1.0, distance / sun_radius, 1.0);
    }

    let start = vec2<i32>(i32(in.uv.x * f32(texture_dimensions.x)), i32(in.uv.y * f32(texture_dimensions.y)));
    let end = vec2<i32>(i32(mouse_position.x * f32(texture_dimensions.x)), i32(mouse_position.y * f32(texture_dimensions.y)));
    let shadow = blocked_line_of_sight(start, end, texture_dimensions);

    if shadow > 0.001 {
        let lightness = 1.0 - shadow;
        colour = vec4<f32>(lightness, lightness, lightness, 1.0);
    }


    // return quad_colour * textureSample(height_map, height_map_sampler, in.uv) * textureSample(colour_map, colour_map_sampler, in.uv) * COLOUR_MULTIPLIER;
    // return quad_colour * textureSample(height_map, height_map_sampler, in.uv) * COLOUR_MULTIPLIER;
    return quad_colour * textureSample(colour_map, colour_map_sampler, in.uv) * colour ;
}