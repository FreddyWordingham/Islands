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
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .insert_resource(Terrain::new())
        .add_event::<RegenerateTerrain>()
        .add_event::<RedrawTerrain>()
        .add_systems(Update, input_events)
        .add_systems(Update, regenerate_terrain.after(input_events))
        .add_systems(Update, redraw_colour_map.after(regenerate_terrain))
        .add_systems(Update, redraw_height_map.after(regenerate_terrain))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    // asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    // mut events: EventWriter<RegenerateTerrain>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // // Load or create the texture
    // let texture_handle = asset_server.load("textures/blank.png");

    // If you need to create a texture dynamically
    let texture_size = Extent3d {
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
        depth_or_array_layers: 1,
    };

    // Height map
    let height_map = Image {
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
        data: vec![255; (MAP_WIDTH * MAP_HEIGHT * 4) as usize],
        ..Default::default()
    };
    let height_map_handle = images.add(height_map);

    // Colour map
    let colour_map = Image {
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
        data: vec![255; (MAP_WIDTH * MAP_HEIGHT * 4) as usize],
        ..Default::default()
    };
    let colour_map_handle = images.add(colour_map);

    // Rendering quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(1.0, 1.0))).into(),
            transform: Transform::default().with_scale(Vec3::splat(400.0)),
            material: materials.add(CustomMaterial::new(
                Some(height_map_handle),
                Some(colour_map_handle),
            )),
            ..Default::default()
        },
        Canvas,
    ));

    // // Generate terrain
    // events.send(RegenerateTerrain);
}
