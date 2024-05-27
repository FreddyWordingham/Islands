use bevy::{prelude::*, window::PrimaryWindow};

use crate::prelude::*;

pub fn print_mouse_position(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    let coords = get_cursor_coords(window, camera, camera_transform);
    println!("{:?}", coords);
}

pub fn get_cursor_coords(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let x = (world_position.x / RENDER_WIDTH) + 0.5;
        let y = (world_position.y / RENDER_HEIGHT) + 0.5;

        return Some(Vec2::new(x, y));
    }
    return None;
}

pub fn update_sun_position(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    query: Query<&Handle<CustomMaterial>>,
    mut material_handle: ResMut<Assets<CustomMaterial>>,
) {
    for material in query.iter() {
        let material_id = material.id();
        let material = material_handle.get_mut(material_id).unwrap();

        let sun_position = material.mouse_position.as_mut();
        let (camera, camera_transform) = camera.single();
        let coords = get_cursor_coords(window.single(), camera, camera_transform);

        if let Some(coords) = coords {
            sun_position[0] = coords.x;
            sun_position[1] = 1.0 - coords.y;
        }
    }
}
