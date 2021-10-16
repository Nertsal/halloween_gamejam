use super::*;

pub struct Knight {
    pub circle: Circle,
    pub speed: f32,
    pub velocity: Velocity,
    pub sprite: Sprite,
    pub health: Health,
}

impl Knight {
    pub fn new(
        circle: Circle,
        speed: f32,
        health: Health,
        acceleration: f32,
        texture: &Texture,
    ) -> Self {
        Self {
            circle,
            speed,
            velocity: Velocity::new(acceleration),
            sprite: texture.into(),
            health,
        }
    }

    pub fn target(&mut self, target_pos: Vec2<f32>) {
        let direction = target_pos - self.circle.position;
        self.velocity.target = direction.clamp(self.speed);
    }
}

impl GameState {
    pub fn spawn_knight(&mut self, position: Vec2<f32>) {
        let knight = Knight::new(
            Circle::new(position, constants::KNIGHT_RADIUS),
            constants::KNIGHT_SPEED,
            Health::new(constants::KNIGHT_HEALTH),
            constants::KNIGHT_ACCELERATION,
            &self.assets.sprites.knight,
        );
        self.knights.push(knight);
    }
}
