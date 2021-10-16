use super::*;

pub type Mana = Health;

pub struct Player {
    pub circle: Circle,
    pub speed: f32,
    pub velocity: Vec2<f32>,
    pub sprite: Sprite,
    pub health: Health,
    pub mana: Mana,
}

impl Player {
    pub fn new(circle: Circle, speed: f32, health: Health, mana: Mana, texture: &Texture) -> Self {
        Self {
            circle,
            speed,
            velocity: Vec2::ZERO,
            sprite: texture.into(),
            health,
            mana,
        }
    }
}
