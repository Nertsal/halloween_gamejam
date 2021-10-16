use super::*;

pub struct Velocity {
    pub current: Vec2<f32>,
    pub target: Vec2<f32>,
    pub acceleration: f32,
}

impl Velocity {
    pub fn new(acceleration: f32) -> Self {
        Self {
            current: Vec2::ZERO,
            target: Vec2::ZERO,
            acceleration,
        }
    }

    pub fn accelerate(&mut self, delta_time: f32) {
        self.current += (self.target - self.current).clamp(self.acceleration * delta_time);
    }
}
