use super::*;

#[derive(Copy, Clone)]
pub enum SkeletonType {
    Warrior,
    Archer,
}

pub enum SkeletonState {
    Spawning { time_left: f32 },
    Alive,
}

pub struct SkeletonWarrior {
    pub circle: Circle,
    pub speed: f32,
    pub velocity: Velocity,
    pub sprite: Sprite,
    pub state: SkeletonState,
    pub health: Health,
}

impl SkeletonWarrior {
    pub fn new(
        circle: Circle,
        speed: f32,
        spawn_time: f32,
        health: Health,
        acceleration: f32,
        texture: &Texture,
    ) -> Self {
        Self {
            circle,
            speed,
            velocity: Velocity::new(acceleration),
            sprite: texture.into(),
            state: SkeletonState::Spawning {
                time_left: spawn_time,
            },
            health,
        }
    }

    pub fn target(&mut self, target_pos: Vec2<f32>) {
        let direction = target_pos - self.circle.position;
        self.velocity.target = direction.clamp(self.speed);
    }
}

pub struct SkeletonArcher {
    pub circle: Circle,
    pub speed: f32,
    pub velocity: Velocity,
    pub sprite: Sprite,
    pub state: SkeletonState,
    pub health: Health,
    pub shoot_timer: f32,
    pub shoot_cooldown: f32,
}

impl SkeletonArcher {
    pub fn new(
        circle: Circle,
        speed: f32,
        spawn_time: f32,
        health: Health,
        acceleration: f32,
        shoot_cooldown: f32,
        texture: &Texture,
    ) -> Self {
        Self {
            circle,
            speed,
            velocity: Velocity::new(acceleration),
            sprite: texture.into(),
            state: SkeletonState::Spawning {
                time_left: spawn_time,
            },
            health,
            shoot_cooldown,
            shoot_timer: shoot_cooldown,
        }
    }

    pub fn target(&mut self, target_pos: Vec2<f32>) {
        let direction = target_pos - self.circle.position;
        self.velocity.target = direction.clamp(self.speed);
    }
}

impl GameState {
    pub fn spawn_skeleton_warrior(&mut self, position: Vec2<f32>) {
        let skeleton = SkeletonWarrior::new(
            Circle::new(position, constants::SKELETON_RADIUS),
            constants::SKELETON_SPEED,
            constants::SKELETON_SPAWN_TIME,
            Health::new(constants::SKELETON_HEALTH),
            constants::SKELETON_ACCELERATION,
            &self.assets.sprites.skeleton,
        );
        self.skeletons_warriors.push(skeleton);
    }

    pub fn spawn_skeleton_archer(&mut self, position: Vec2<f32>) {
        let skeleton = SkeletonArcher::new(
            Circle::new(position, constants::SKELETON_RADIUS),
            constants::SKELETON_SPEED,
            constants::SKELETON_SPAWN_TIME,
            Health::new(constants::SKELETON_HEALTH),
            constants::SKELETON_ACCELERATION,
            constants::SKELETON_ARCHER_COOLDOWN,
            &self.assets.sprites.skeleton,
        );
        self.skeletons_archers.push(skeleton);
    }
}
