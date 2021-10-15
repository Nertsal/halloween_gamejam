use super::*;

pub struct Knight {
    pub circle: Circle,
    pub speed: f32,
    pub velocity: Vec2<f32>,
    pub texture: Texture,
}

impl Knight {
    pub fn new(circle: Circle, speed: f32, texture: &Texture) -> Self {
        Self {
            circle,
            speed,
            velocity: Vec2::ZERO,
            texture: texture.clone(),
        }
    }
}

impl GameState {
    pub fn spawn_knight(&mut self, position: Vec2<f32>) {
        let knight = Knight::new(
            Circle::new(position, constants::KNIGHT_RADIUS),
            constants::KNIGHT_SPEED,
            &self.assets.sprites.knight,
        );
        self.knights.push(knight);
    }
}
