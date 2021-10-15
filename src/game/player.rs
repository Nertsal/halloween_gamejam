use super::*;

pub struct Player {
    pub position: Vec2<f32>,
    pub speed: f32,
    pub velocity: Vec2<f32>,
    pub radius: f32,
    pub texture: Texture,
}

impl Player {
    pub fn new(position: Vec2<f32>, speed: f32, radius: f32, texture: &Texture) -> Self {
        Self {
            position,
            speed,
            radius,
            velocity: Vec2::ZERO,
            texture: texture.clone(),
        }
    }
}
