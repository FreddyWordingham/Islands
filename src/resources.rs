use bevy::{
    math::{vec2, Vec2},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use ndarray::Array2;
use rand::prelude::*;

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

#[derive(Resource)]
pub struct PerlinNoise {
    pub vectors: Array2<Vec2>,
}

impl Default for PerlinNoise {
    fn default() -> Self {
        let mut rng = thread_rng();

        let width = 5;
        let height = 5;
        let mut vectors = Array2::default((height, width));
        for i in 0..width {
            for j in 0..height {
                let theta = rng.gen_range(0.0..std::f32::consts::TAU);
                let x = theta.cos();
                let y = theta.sin();
                vectors[(i, j)] = vec2(x, y);
            }
        }

        Self { vectors }
    }
}

impl PerlinNoise {
    pub fn randomise(&mut self) {
        let mut rng = thread_rng();

        let width = self.vectors.nrows();
        let height = self.vectors.ncols();
        for i in 0..width {
            for j in 0..height {
                let theta = rng.gen_range(0.0..std::f32::consts::TAU);
                let x = theta.cos();
                let y = theta.sin();
                self.vectors[(i, j)] = vec2(x, y);
            }
        }
    }

    pub fn render(&self, data: &mut [u8], width: u32, height: u32) {
        for i in 0..width {
            let x = i as f32 / width as f32;
            for j in 0..height {
                let y = j as f32 / height as f32;
                let position = vec2(x, y);

                let sample = self.sample(position);
                let idx = ((j * width + i) * 4) as usize;

                data[idx] = (sample * 255.0) as u8;
                data[idx + 1] = (sample * 255.0) as u8;
                data[idx + 2] = (sample * 255.0) as u8;
                data[idx + 3] = 255;
            }
        }
    }

    pub fn sample(&self, position: Vec2) -> f32 {
        let height = self.vectors.nrows() as i32;
        let width = self.vectors.ncols() as i32;

        let left = (width as f32 * position.x).floor() as i32 % width;
        let right = (left + 1) % width;
        let top = (height as f32 * position.y).floor() as i32 % height;
        let bottom = (top + 1) % height;

        let top_left = self.vectors[(top as usize, left as usize)];
        let top_right = self.vectors[(top as usize, right as usize)];
        let bottom_left = self.vectors[(bottom as usize, left as usize)];
        let bottom_right = self.vectors[(bottom as usize, right as usize)];

        let xf = (width as f32 * position.x).fract();
        let yf = (height as f32 * position.y).fract();

        // Correct relative position vectors for gradients
        let top_left_gradient = top_left.dot(vec2(xf, yf));
        let top_right_gradient = top_right.dot(vec2(xf - 1.0, yf));
        let bottom_left_gradient = bottom_left.dot(vec2(xf, yf - 1.0));
        let bottom_right_gradient = bottom_right.dot(vec2(xf - 1.0, yf - 1.0));

        let top_gradient = Self::lerp(top_left_gradient, top_right_gradient, xf);
        let bottom_gradient = Self::lerp(bottom_left_gradient, bottom_right_gradient, xf);

        Self::lerp(top_gradient, bottom_gradient, yf)
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        let t = Self::smoothstep(t);
        a * (1.0 - t) + b * t
    }

    fn smoothstep(t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t)
    }
}
