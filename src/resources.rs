use bevy::prelude::*;
use ndarray::Array2;

use crate::prelude::*;

#[derive(Resource)]
pub struct Terrain {
    pub height_map: Array2<f32>,
}

impl Terrain {
    pub fn new() -> Self {
        Self {
            height_map: Array2::zeros((MAP_HEIGHT as usize, MAP_WIDTH as usize)),
        }
    }
}
