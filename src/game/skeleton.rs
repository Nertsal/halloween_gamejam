use super::*;

pub struct Skeleton {
    pub position: Vec2<f32>,
    pub radius: f32,
    pub speed: f32,
    pub velocity: Vec2<f32>,
    pub texture: Texture,
    pub state: SkeletonState,
}

pub enum SkeletonState {
    Spawning { time_left: f32 },
    Alive,
}

impl Skeleton {
    pub fn new(
        position: Vec2<f32>,
        radius: f32,
        speed: f32,
        spawn_time: f32,
        texture: &Texture,
    ) -> Self {
        Self {
            position,
            radius,
            speed,
            velocity: Vec2::ZERO,
            texture: texture.clone(),
            state: SkeletonState::Spawning {
                time_left: spawn_time,
            },
        }
    }
}

impl GameState {
    pub fn spawn_skeleton(&mut self, position: Vec2<f32>) {
        let skeleton = Skeleton::new(
            position,
            constants::SKELETON_RADIUS,
            constants::SKELETON_SPEED,
            constants::SKELETON_SPAWN_TIME,
            &self.assets.sprites.skeleton,
        );
        self.skeletons.push(skeleton);
    }
}
