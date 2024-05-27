use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// Data is passed to the shader.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub mouse_position: Vec2,
    #[uniform(1)]
    pub quad_colour: Color,
    #[texture(2)]
    #[sampler(3)]
    pub height_map: Option<Handle<Image>>,
    #[texture(4)]
    #[sampler(5)]
    pub colour_map: Option<Handle<Image>>,
}

impl CustomMaterial {
    pub fn new(height_map: Option<Handle<Image>>, colour_map: Option<Handle<Image>>) -> Self {
        Self {
            mouse_position: Vec2::new(0.5, 0.5),
            quad_colour: Color::WHITE,
            height_map: height_map,
            colour_map: colour_map,
        }
    }
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain.wgsl".into()
    }
}
