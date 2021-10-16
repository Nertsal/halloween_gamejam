use super::*;

pub const BACKGROUND_COLOR: Color<f32> = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};

pub const FIRST_SPAWN_DELAY: f32 = 3.0;
pub const SPAWN_DELAY: f32 = 1.0;

pub const PLAYER_SPEED: f32 = 10.0;
pub const PLAYER_RADIUS: f32 = 2.5;
pub const PLAYER_HEALTH: f32 = 100.0;
pub const PLAYER_MANA: f32 = 100.0;
pub const PLAYER_HIT_STRENGTH: f32 = 10.0;
pub const PLAYER_HIT_FORCE: f32 = 25.0;

pub const SPELL_WARRIOR_COST: f32 = 10.0;
pub const SPELL_ARCHER_COST: f32 = 20.0;
pub const SPELL_FIREBALL_COST: f32 = 50.0;

pub const SKELETON_SPEED: f32 = 12.5;
pub const SKELETON_RADIUS: f32 = 2.5;
pub const SKELETON_SPAWN_TIME: f32 = 0.5;
pub const SKELETON_HEALTH: f32 = 10.0;
pub const SKELETON_HIT_STRENGTH: f32 = 10.0;
pub const SKELETON_ACCELERATION: f32 = 10.0;

pub const SKELETON_ARCHER_DISTANCE: f32 = 25.0;
pub const SKELETON_ARCHER_COOLDOWN: f32 = 3.0;
pub const SKELETON_ARCHER_RANDOMNESS: f32 = 2.0;

pub const ARROW_SPEED: f32 = 20.0;
pub const ARROW_RADIUS: f32 = 2.0;
pub const ARROW_HIT_STRENGTH: f32 = 10.0;
pub const ARROW_HIT_FORCE: f32 = 10.0;

pub const KNIGHT_SPEED: f32 = 7.5;
pub const KNIGHT_RADIUS: f32 = 2.5;
pub const KNIGHT_HEALTH: f32 = 20.0;
pub const KNIGHT_HIT_STRENGTH: f32 = 10.0;
pub const KNIGHT_HIT_FORCE: f32 = 5.0;
pub const KNIGHT_ACCELERATION: f32 = 10.0;

pub const PARTICLE_ALPHA: f32 = 0.5;
pub const PARTICLE_DECAY_SPEED: f32 = 0.5;

pub const CASTLE_SIZE: f32 = 12.0;
pub const GRAVE_SIZE: f32 = 2.0;

pub const SPELL_RING_RADIUS: f32 = 10.0;
pub const SPELLS_MARGIN: f32 = 10.0;
pub const SPELL_POINT_RADIUS: f32 = 2.0;
pub const SPELL_CONNECTION_WIDTH: f32 = 0.5;

pub const KNIGHT_KILL_MANA: f32 = 30.0;
