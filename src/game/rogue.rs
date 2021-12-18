use super::*;

pub struct Rogue {
    pub circle: Circle,
    pub speed: f32,
    pub velocity: Velocity,
    pub sprite: Sprite,
    pub health: Health,
}

impl Rogue {
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
        self.velocity.target = direction.clamp_len(..=self.speed);
    }
}

impl GameState {
    pub fn spawn_rogue(&mut self, position: Vec2<f32>) {
        let rogue = Rogue::new(
            Circle::new(position, constants::ROGUE_RADIUS),
            constants::ROGUE_SPEED,
            Health::new(constants::ROGUE_HEALTH),
            constants::ROGUE_ACCELERATION,
            &self.assets.sprites.rogue,
        );
        self.rogues.push(rogue);
    }
}
