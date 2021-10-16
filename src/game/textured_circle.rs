use super::*;

pub struct TexturedCircle {
    pub circle: Circle,
    pub sprite: Sprite,
}

impl TexturedCircle {
    pub fn new(circle: Circle, texture: &Texture) -> Self {
        Self {
            circle,
            sprite: Sprite::new(texture),
        }
    }

    pub fn bottom(&self) -> Vec2<f32> {
        self.circle.position - vec2(0.0, self.circle.radius)
    }
}
