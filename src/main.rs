use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

use islands::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, check_spacebar)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    // asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Load or create the texture
    // let texture_handle = asset_server.load("textures/blank.png");

    // If you need to create a texture dynamically
    let width = 256;
    let height = 256;
    let texture_size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let texture = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: texture_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        data: vec![255; (width * height * 4) as usize],
        ..Default::default()
    };
    let texture_handle = images.add(texture);

    // Rendering quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(1.0, 1.0))).into(),
            transform: Transform::default().with_scale(Vec3::splat(200.0)),
            material: materials.add(CustomMaterial::new(Some(texture_handle))),
            ..Default::default()
        },
        Terrain,
    ));
}

fn check_spacebar(
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

            draw_horizontal_line(texture, 128);
        }
        println!("Spacebar pressed");
    }
}

fn draw_horizontal_line(image: &mut Image, y: u32) {
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
