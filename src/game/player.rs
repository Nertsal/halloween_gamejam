use super::*;

pub struct Player {
    pub circle: Circle,
    pub speed: f32,
    pub velocity: Vec2<f32>,
    pub texture: Texture,
    pub health: Health,
}

impl Player {
    pub fn new(circle: Circle, speed: f32, health: Health, texture: &Texture) -> Self {
        Self {
            circle,
            speed,
            velocity: Vec2::ZERO,
            texture: texture.clone(),
            health,
        }
    }
}
