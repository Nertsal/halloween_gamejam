use super::*;

pub struct Castle {
    pub circle: Circle,
    pub sprite: Sprite,
}

impl Castle {
    pub fn new(circle: Circle, texture: &Texture) -> Self {
        Self {
            circle,
            sprite: Sprite::new(texture),
        }
    }

    pub fn spawn_position(&self) -> Vec2<f32> {
        self.circle.position - vec2(0.0, self.circle.radius)
    }
}
