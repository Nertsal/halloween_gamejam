use super::*;

pub struct Circle {
    pub position: Vec2<f32>,
    pub radius: f32,
}

impl Circle {
    pub fn new(position: Vec2<f32>, radius: f32) -> Self {
        Self { position, radius }
    }
}
