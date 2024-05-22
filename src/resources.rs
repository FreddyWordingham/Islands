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
    pub quad_colour: Color,
    #[texture(1)]
    #[sampler(2)]
    pub quad_texture: Option<Handle<Image>>,
}

impl CustomMaterial {
    pub fn new(texture: Option<Handle<Image>>) -> Self {
        Self {
            quad_colour: Color::WHITE,
            quad_texture: texture,
        }
    }
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain.wgsl".into()
    }
}
