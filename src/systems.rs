use bevy::prelude::*;

use crate::prelude::*;

pub fn randomise_terrain(
    query: Query<&Handle<CustomMaterial>>,
    mut material_handle: ResMut<Assets<CustomMaterial>>,
    mut texture_handle: ResMut<Assets<Image>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut noise: ResMut<PerlinNoise>,
) {
    if keyboard_input.pressed(KeyCode::KeyR) {
        noise.randomise();

        for material in query.iter() {
            let material_id = material.id();
            let material = material_handle.get_mut(material_id).unwrap();

            let texture_id = material.quad_texture.as_ref().unwrap().id();
            let texture = texture_handle.get_mut(texture_id).unwrap();

            noise.render(
                &mut texture.data,
                texture.texture_descriptor.size.width,
                texture.texture_descriptor.size.height,
            );
        }
    }
}

pub fn generate_terrain(
    query: Query<&Handle<CustomMaterial>>,
    mut material_handle: ResMut<Assets<CustomMaterial>>,
    mut texture_handle: ResMut<Assets<Image>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    noise: Res<PerlinNoise>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for material in query.iter() {
            let material_id = material.id();
            let material = material_handle.get_mut(material_id).unwrap();

            let texture_id = material.quad_texture.as_ref().unwrap().id();
            let texture = texture_handle.get_mut(texture_id).unwrap();

            noise.render(
                &mut texture.data,
                texture.texture_descriptor.size.width,
                texture.texture_descriptor.size.height,
            );
        }
    }
}

pub fn check_spacebar(
    query: Query<&Handle<CustomMaterial>>,
    mut material_handle: ResMut<Assets<CustomMaterial>>,
    mut texture_handle: ResMut<Assets<Image>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for material in query.iter() {
            let material_id = material.id();
            let material = material_handle.get_mut(material_id).unwrap();

            let texture_id = material.quad_texture.as_ref().unwrap().id();
            let texture = texture_handle.get_mut(texture_id).unwrap();

            draw_horizontal_line(texture, 64);
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
