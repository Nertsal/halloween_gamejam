use super::*;

pub struct Projectile {
    pub circle: Circle,
    pub velocity: Vec2<f32>,
    pub hit: bool,
    pub texture: Texture,
    pub typ: ProjectileType,
}

impl Projectile {
    pub fn new(circle: Circle, velocity: Vec2<f32>, texture: &Texture, typ: ProjectileType) -> Self {
        Self {
            circle,
            velocity,
            hit: false,
            texture: texture.clone(),
            typ,
        }
    }
}

pub enum ProjectileType {
    Arrow,
    Fireball,
}
