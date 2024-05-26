use bevy::{math::vec2, prelude::*};
use ndarray::Array2;
use rand::prelude::*;
use std::f32::consts::{SQRT_2, TAU};

pub struct PerlinNoise {
    pub vector_layers: Vec<(Array2<Vec2>, f32)>,
}

// impl Default for PerlinNoise {
//     fn default() -> Self {
//         // Generate random vector layers
//         let mut rng = thread_rng();
//         let layer_sizes = vec![
//             ((3, 3), 1.0), //
//             ((5, 5), 0.8), //
//             ((7, 7), 0.5),
//             ((13, 13), 0.4),
//         ];
//         let mut vector_layers: Vec<_> = layer_sizes
//             .iter()
//             .map(|((width, height), weight)| {
//                 (generate_vector_layer(&mut rng, *width, *height), *weight)
//             })
//             .collect();

//         // Normalise layer weights
//         let total_weight: f32 = vector_layers.iter().map(|(_, weight)| weight).sum();
//         for (_, weight) in &mut vector_layers {
//             *weight /= total_weight;
//         }

//         Self { vector_layers }
//     }
// }

impl PerlinNoise {
    pub fn new(layers: Vec<((usize, usize), f32)>) -> Self {
        let vector_layers = layers
            .iter()
            .map(|((width, height), weight)| {
                (Array2::from_elem((*height, *width), Vec2::ZERO), *weight)
            })
            .collect();

        let mut perlin_noise = Self { vector_layers };
        perlin_noise.randomise();

        return perlin_noise;
    }

    /// Randomly orientate all vectors in all layers.
    pub fn randomise(&mut self) {
        let mut rng = thread_rng();
        for (vectors, _weight) in &mut self.vector_layers {
            let width = vectors.ncols();
            let height = vectors.nrows();
            for yi in 0..height {
                for xi in 0..width {
                    let theta = rng.gen_range(0.0..TAU);
                    let x = SQRT_2 * theta.cos();
                    let y = SQRT_2 * theta.sin();
                    vectors[(yi, xi)] = vec2(x, y);
                }
            }
        }
    }

    /// Returns a value between 0 and 1.
    pub fn sample(&self, position: Vec2) -> f32 {
        let mut value = 0.0;
        for (vectors, weight) in &self.vector_layers {
            value += self.sample_layer(vectors, position) * weight;
        }
        value
    }

    /// Returns a value between 0 and 1.
    pub fn sample_layer(&self, layer: &Array2<Vec2>, position: Vec2) -> f32 {
        let height = layer.nrows() as i32;
        let width = layer.ncols() as i32;

        let left = (width as f32 * position.x).floor() as i32 % width;
        let right = (left + 1) % width;
        let top = (height as f32 * position.y).floor() as i32 % height;
        let bottom = (top + 1) % height;

        let top_left = layer[(top as usize, left as usize)];
        let top_right = layer[(top as usize, right as usize)];
        let bottom_left = layer[(bottom as usize, left as usize)];
        let bottom_right = layer[(bottom as usize, right as usize)];

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
