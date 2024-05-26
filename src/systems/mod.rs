use bevy::{math::vec2, prelude::*};
use ndarray::Array2;
use ndarray_stats::QuantileExt;

use crate::prelude::*;

pub fn input_events(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<RegenerateTerrain>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        events.send(RegenerateTerrain);
    }
}

pub fn regenerate_terrain(
    mut regenerate_terrain_events: EventReader<RegenerateTerrain>,
    mut redraw_terrain_events: EventWriter<RedrawTerrain>,
    mut terrain: ResMut<Terrain>,
) {
    for _ in regenerate_terrain_events.read() {
        // Generate hills
        let perlin_noise_generator = PerlinNoise::new(vec![
            ((3, 3), 1.0),
            ((5, 5), 0.7),
            ((7, 7), 0.5),
            ((11, 11), 0.3),
            ((13, 13), 0.2),
        ]);
        for yi in 0..MAP_HEIGHT {
            let y = yi as f32 / MAP_HEIGHT as f32;
            for xi in 0..MAP_WIDTH {
                let x = xi as f32 / MAP_WIDTH as f32;
                let height = perlin_noise_generator.sample(vec2(x as f32, y as f32));
                terrain.height_map[(yi as usize, xi as usize)] = height;
            }
        }

        // Normalize the height map
        let min_value = *terrain.height_map.min().unwrap();
        let max_value = *terrain.height_map.max().unwrap();
        terrain
            .height_map
            .mapv_inplace(|x| (x - min_value) / (max_value - min_value));

        // Circular island
        let center = vec2(MAP_WIDTH as f32 * 0.5, MAP_HEIGHT as f32 * 0.5);
        let radius = MAP_WIDTH as f32 * 0.25;
        for yi in 0..MAP_HEIGHT {
            for xi in 0..MAP_WIDTH {
                let position = vec2(xi as f32, yi as f32);
                let distance = (position - center).length();
                let scale = (-0.5 * (distance / radius).powi(2)).exp();
                terrain.height_map[(yi as usize, xi as usize)] *= scale;
            }
        }

        // Trigger terrain redraw
        redraw_terrain_events.send(RedrawTerrain);
    }
}

pub fn redraw_height_map(
    mut events: EventReader<RedrawTerrain>,
    query: Query<&Handle<CustomMaterial>>,
    mut material_handle: ResMut<Assets<CustomMaterial>>,
    mut texture_handle: ResMut<Assets<Image>>,
    terrain: Res<Terrain>,
) {
    for _ in events.read() {
        for material in query.iter() {
            let material_id = material.id();
            let material = material_handle.get_mut(material_id).unwrap();
            if let Some(height_map_handle) = material.height_map.as_ref() {
                let height_map = texture_handle.get_mut(height_map_handle).unwrap();
                render_height_map(&terrain.height_map, &mut height_map.data);
            }
        }
    }
}

fn render_height_map(height_map: &Array2<f32>, data: &mut [u8]) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let height = height_map[(y as usize, x as usize)];

            let index = (y * MAP_WIDTH + x) as usize * 4;
            data[index] = (height * 255.0) as u8;
            data[index + 1] = (height * 255.0) as u8;
            data[index + 2] = (height * 255.0) as u8;
            data[index + 3] = 255;
        }
    }
}

pub fn redraw_colour_map(
    mut events: EventReader<RedrawTerrain>,
    query: Query<&Handle<CustomMaterial>>,
    mut material_handle: ResMut<Assets<CustomMaterial>>,
    mut texture_handle: ResMut<Assets<Image>>,
    terrain: Res<Terrain>,
) {
    for _ in events.read() {
        for material in query.iter() {
            let material_id = material.id();
            let material = material_handle.get_mut(material_id).unwrap();
            if let Some(colour_map_handle) = material.colour_map.as_ref() {
                let colour_map = texture_handle.get_mut(colour_map_handle).unwrap();
                render_colour_map(&terrain.height_map, &mut colour_map.data);
            }
        }
    }
}

fn render_colour_map(height_map: &Array2<f32>, data: &mut [u8]) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let colour = get_colour(height_map[(y as usize, x as usize)]);

            let index = (y * MAP_WIDTH + x) as usize * 4;
            data[index] = colour[0];
            data[index + 1] = colour[1];
            data[index + 2] = colour[2];
            data[index + 3] = 255;
        }
    }
}

fn get_colour(height: f32) -> [u8; 3] {
    match height {
        0.0..=0.2 => [98, 165, 168],
        0.2..=0.4 => [213, 181, 157],
        0.4..=0.6 => [152, 172, 92],
        0.6..=0.8 => [101, 132, 66],
        0.8..=1.0 => [110, 117, 136],
        1.0..=2.0 => [255, 0, 0],
        _ => [255, 0, 255],
    }
}
