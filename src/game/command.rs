use super::*;

#[derive(Clone)]
pub enum Command {
    Spawn { spawn: CommandSpawn },
}

#[derive(Clone)]
pub enum CommandSpawn {
    Skeleton { skeleton_type: SkeletonType },
    Knight,
}

impl GameState {
    pub fn perform_command(&mut self, command: Command) {
        let mut rng = global_rng();

        match command {
            Command::Spawn { spawn } => match spawn {
                CommandSpawn::Skeleton { skeleton_type } => {
                    let grave = self.graves.choose(&mut rng).unwrap();
                    let position = grave.bottom();
                    self.spawn_skeleton(position, skeleton_type)
                }
                CommandSpawn::Knight => self.spawn_knight(self.castle.bottom()),
            },
        }
    }
}
