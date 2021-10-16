use super::*;

#[derive(Clone)]
pub enum Command {
    Spawn { spawn: CommandSpawn },
}

#[derive(Clone)]
pub enum CommandSpawn {
    Skeleton { skeleton_type: SkeletonType },
}

impl GameState {
    pub fn perform_command(&mut self, command: Command) {
        let mut rng = global_rng();

        match command {
            Command::Spawn { spawn } => match spawn {
                CommandSpawn::Skeleton { skeleton_type } => {
                    let grave = self.graves.choose(&mut rng).unwrap();
                    let position = grave.bottom();

                    match skeleton_type {
                        SkeletonType::Warrior => {
                            if !self.player.mana.try_change(-constants::SPELL_WARRIOR_COST) {
                                return;
                            }
                            self.spawn_skeleton_warrior(position);
                        }
                        SkeletonType::Archer => {
                            if !self.player.mana.try_change(-constants::SPELL_ARCHER_COST) {
                                return;
                            }
                            self.spawn_skeleton_archer(position);
                        }
                    };
                }
            },
        }
    }
}
