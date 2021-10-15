use super::*;

pub struct Skeleton {
    pub position: Vec2<f32>,
    pub radius: f32,
    pub speed: f32,
    pub velocity: Vec2<f32>,
    pub texture: Texture,
}

impl Skeleton {
    pub fn new(position: Vec2<f32>, radius: f32, speed: f32, texture: &Texture) -> Self {
        Self {
            position,
            radius,
            speed,
            velocity: Vec2::ZERO,
            texture: texture.clone(),
        }
    }
}

impl GameState {
    pub fn spawn_skeleton(&mut self, position: Vec2<f32>) {
        let skeleton = Skeleton::new(
            position,
            constants::SKELETON_RADIUS,
            constants::SKELETON_SPEED,
            &self.assets.sprites.skeleton,
        );
        self.skeletons.push(skeleton);
    }
}
