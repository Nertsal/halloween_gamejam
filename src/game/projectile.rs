use super::*;

pub struct Projectile {
    pub circle: Circle,
    pub velocity: Vec2<f32>,
    pub hit: bool,
    pub texture: Texture,
}

impl Projectile {
    pub fn new(circle: Circle, velocity: Vec2<f32>, texture: &Texture) -> Self {
        Self {
            circle,
            velocity,
            hit: false,
            texture: texture.clone(),
        }
    }
}
