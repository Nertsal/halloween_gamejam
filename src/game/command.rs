use super::*;

pub enum Command {
    Spawn {
        position: Vec2<f32>,
        spawn: CommandSpawn,
    },
}

pub enum CommandSpawn {
    Skeleton { skeleton_type: SkeletonType },
    Knight,
}
