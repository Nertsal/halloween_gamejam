use super::*;

#[derive(Clone)]
pub enum Command {
    Spawn {
        position: Vec2<f32>,
        spawn: CommandSpawn,
    },
}

#[derive(Clone)]
pub enum CommandSpawn {
    Skeleton { skeleton_type: SkeletonType },
    Knight,
}

impl GameState {
    pub fn perform_command(&mut self, command: Command) {
        match command {
            Command::Spawn { position, spawn } => match spawn {
                CommandSpawn::Skeleton { skeleton_type } => {
                    self.spawn_skeleton(position, skeleton_type)
                }
                CommandSpawn::Knight => self.spawn_knight(position),
            },
        }
    }
}
