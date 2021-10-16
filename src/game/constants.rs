use super::*;

pub const BACKGROUND_COLOR: Color<f32> = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};

pub const PLAYER_SPEED: f32 = 10.0;
pub const PLAYER_RADIUS: f32 = 2.5;
pub const PLAYER_HEALTH: f32 = 100.0;
pub const PLAYER_HIT_STRENGTH: f32 = 10.0;
pub const PLAYER_HIT_FORCE: f32 = 25.0;

pub const SKELETON_SPEED: f32 = 12.5;
pub const SKELETON_RADIUS: f32 = 2.5;
pub const SKELETON_SPAWN_TIME: f32 = 0.5;
pub const SKELETON_HEALTH: f32 = 10.0;
pub const SKELETON_HIT_STRENGTH: f32 = 10.0;
pub const SKELETON_ACCELERATION: f32 = 10.0;

pub const KNIGHT_SPEED: f32 = 7.5;
pub const KNIGHT_RADIUS: f32 = 2.5;
pub const KNIGHT_HEALTH: f32 = 20.0;
pub const KNIGHT_HIT_STRENGTH: f32 = 10.0;
pub const KNIGHT_HIT_FORCE: f32 = 5.0;
pub const KNIGHT_ACCELERATION: f32 = 10.0;

pub const PARTICLE_ALPHA: f32 = 0.5;
pub const PARTICLE_DECAY_SPEED: f32 = 0.5;
