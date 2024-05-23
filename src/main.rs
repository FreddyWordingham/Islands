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
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .init_resource::<PerlinNoise>()
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        // .add_systems(Update, check_spacebar)
        .add_systems(Update, generate_terrain)
        .add_systems(Update, randomise_terrain)
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

    // // Load or create the texture
    // let texture_handle = asset_server.load("textures/blank.png");

    // If you need to create a texture dynamically
    let width = 128;
    let height = 128;
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
            transform: Transform::default().with_scale(Vec3::splat(400.0)),
            material: materials.add(CustomMaterial::new(Some(texture_handle))),
            ..Default::default()
        },
        Canvas,
    ));
}
