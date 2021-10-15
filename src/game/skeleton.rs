use super::*;

pub struct Skeleton {
    pub circle: Circle,
    pub speed: f32,
    pub velocity: Vec2<f32>,
    pub sprite: Sprite,
    pub state: SkeletonState,
    pub health: Health,
}

pub enum SkeletonState {
    Spawning { time_left: f32 },
    Alive,
}

impl Skeleton {
    pub fn new(
        circle: Circle,
        speed: f32,
        spawn_time: f32,
        health: Health,
        texture: &Texture,
    ) -> Self {
        Self {
            circle,
            speed,
            velocity: Vec2::ZERO,
            sprite: texture.into(),
            state: SkeletonState::Spawning {
                time_left: spawn_time,
            },
            health,
        }
    }
}

impl GameState {
    pub fn spawn_skeleton(&mut self, position: Vec2<f32>) {
        let skeleton = Skeleton::new(
            Circle::new(position, constants::SKELETON_RADIUS),
            constants::SKELETON_SPEED,
            constants::SKELETON_SPAWN_TIME,
            Health::new(constants::SKELETON_HEALTH),
            &self.assets.sprites.skeleton,
        );
        self.skeletons.push(skeleton);
    }
}
