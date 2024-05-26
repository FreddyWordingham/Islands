use bevy::{math::vec2, prelude::*};
use ndarray::Array2;

use crate::prelude::*;

#[derive(Event)]
pub struct HeightMapUpdatedEvent;

// The system that updates the height map and sends an event
pub fn update_height_map(
    query: Query<&Handle<CustomMaterial>>,
    mut material_handle: ResMut<Assets<CustomMaterial>>,
    mut texture_handle: ResMut<Assets<Image>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    noise: Res<PerlinNoise>,
    mut event_writer: EventWriter<HeightMapUpdatedEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for material in query.iter() {
            let material_id = material.id();
            let material = material_handle.get_mut(material_id).unwrap();

            if let Some(height_map_handle) = material.height_map.as_ref() {
                let height_map = texture_handle.get_mut(height_map_handle).unwrap();

                render_height(
                    &noise,
                    &mut height_map.data,
                    height_map.texture_descriptor.size.width,
                    height_map.texture_descriptor.size.height,
                );

                event_writer.send(HeightMapUpdatedEvent);
            }
        }
    }
}

// The system that updates the colour map after the height map has been updated
pub fn update_colour_map(
    query: Query<&Handle<CustomMaterial>>,
    mut material_handle: ResMut<Assets<CustomMaterial>>,
    mut texture_handle: ResMut<Assets<Image>>,
    noise: Res<PerlinNoise>,
    mut event_reader: EventReader<HeightMapUpdatedEvent>,
) {
    for _ in event_reader.read() {
        for material in query.iter() {
            let material_id = material.id();
            let material = material_handle.get_mut(material_id).unwrap();

            if let Some(colour_map_handle) = material.colour_map.as_ref() {
                let colour_map = texture_handle.get_mut(colour_map_handle).unwrap();

                render_colour(
                    &noise,
                    &mut colour_map.data,
                    colour_map.texture_descriptor.size.width,
                    colour_map.texture_descriptor.size.height,
                );
            }
        }
    }
}

pub fn render_height(perlin_noise: &PerlinNoise, height_map: &mut [u8], width: u32, height: u32) {
    let mut landscape = Array2::zeros((height as usize, width as usize));

    // Noise height map
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for j in 0..height as usize {
        let y = j as f32 / height as f32;
        for i in 0..width as usize {
            let x = i as f32 / width as f32;
            let position = vec2(x, y);
            landscape[(j, i)] = perlin_noise.sample(position);

            if landscape[(j, i)] < min {
                min = landscape[(j, i)];
            }
            if landscape[(j, i)] > max {
                max = landscape[(j, i)];
            }
        }
    }

    // Normalise
    for j in 0..height as usize {
        for i in 0..width as usize {
            landscape[(j, i)] = (landscape[(j, i)] - min) / (max - min);
        }
    }

    let channels = 4;
    for yi in 0..height {
        for xi in 0..width {
            let idx = (yi * width + xi) as usize * channels;
            let value = landscape[(yi as usize, xi as usize)];
            height_map[idx] = (value * 255.0) as u8;
            height_map[idx + 1] = (value * 255.0) as u8;
            height_map[idx + 2] = (value * 255.0) as u8;
            height_map[idx + 3] = 255;
        }
    }
}

pub fn render_colour(perlin_noise: &PerlinNoise, colour_map: &mut [u8], width: u32, height: u32) {
    let mut landscape = Array2::zeros((height as usize, width as usize));

    // Noise heightmap
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for j in 0..height as usize {
        let y = j as f32 / height as f32;
        for i in 0..width as usize {
            let x = i as f32 / width as f32;
            let position = vec2(x, y);
            landscape[(j, i)] = perlin_noise.sample(position);

            if landscape[(j, i)] < min {
                min = landscape[(j, i)];
            }
            if landscape[(j, i)] > max {
                max = landscape[(j, i)];
            }
        }
    }

    // Normalise
    for j in 0..height as usize {
        for i in 0..width as usize {
            landscape[(j, i)] = (landscape[(j, i)] - min) / (max - min);
        }
    }

    // Colour map
    let channels = 4;
    for yi in 0..height {
        for xi in 0..width {
            let idx = (yi * width + xi) as usize * channels;
            let value = landscape[(yi as usize, xi as usize)];

            let colour = match value {
                -1.0..=0.0 => [0, 0, 153],
                0.0..=0.2 => [98, 165, 168],
                0.2..=0.4 => [213, 181, 157],
                0.4..=0.6 => [152, 172, 92],
                0.6..=0.8 => [101, 132, 66],
                0.8..=1.0 => [110, 117, 136],
                1.0..=2.0 => [255, 0, 0],
                _ => [0, 0, 0],
            };

            colour_map[idx] = colour[0];
            colour_map[idx + 1] = colour[1];
            colour_map[idx + 2] = colour[2];
            colour_map[idx + 3] = 255;
        }
    }
}

pub fn render_landscape(
    perlin_noise: &PerlinNoise,
    height_map: &mut [u8],
    width: u32,
    height: u32,
) {
    let mut landscape = Array2::zeros((height as usize, width as usize));

    for j in 0..height as usize {
        let y = j as f32 / height as f32;
        for i in 0..width as usize {
            let x = i as f32 / width as f32;
            let position = vec2(x, y);
            landscape[(j, i)] = perlin_noise.sample(position);
        }
    }

    /// Circular fall-off
    let channels = 4; // Assuming RGBA8 format
    for yi in 0..height {
        let y = yi as f32 / height as f32;
        let y = (y * 2.0) - 1.0;
        for xi in 0..width {
            let x = xi as f32 / width as f32;
            let x = (x * 2.0) - 1.0;

            let r = (1.0 - (x * x + y * y)).max(0.0);

            let idx = (yi * width + xi) as usize * channels;
            height_map[idx] = (height_map[idx] as f32 * r) as u8;
            height_map[idx + 1] = (height_map[idx + 1] as f32 * r) as u8;
            height_map[idx + 2] = (height_map[idx + 2] as f32 * r) as u8;
        }
    }
}

pub fn draw_horizontal_line(image: &mut Image, y: u32) {
    let width = image.texture_descriptor.size.width;
    let height = image.texture_descriptor.size.height;
    let channels = 4; // Assuming RGBA8 format

    if y >= height {
        return; // Out of bounds check
    }

    for x in 0..width {
        let idx = (y * width + x) as usize * channels;
        image.data[idx] = 150; // Red
        image.data[idx + 1] = 150; // Green
        image.data[idx + 2] = 150; // Blue
        image.data[idx + 3] = 150; // Alpha
    }
}
