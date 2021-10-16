use super::*;

#[derive(Clone)]
pub enum Command {
    Spawn { spawn: CommandSpawn },
}

#[derive(Clone)]
pub enum CommandSpawn {
    Skeleton { skeleton_type: SkeletonType },
    Fireball,
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
                CommandSpawn::Fireball => {
                    let target = self
                        .knights
                        .iter()
                        .map(|knight| {
                            (
                                knight,
                                (knight.circle.position - self.player.circle.position).len(),
                            )
                        })
                        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
                    if let Some((target, distance)) = target {
                        if !self.player.mana.try_change(-constants::SPELL_FIREBALL_COST) {
                            return;
                        }

                        let direction =
                            (target.circle.position - self.player.circle.position) / distance;
                        let fireball = Projectile::new(
                            Circle::new(self.player.circle.position, constants::FIREBALL_RADIUS),
                            direction * constants::FIREBALL_SPEED,
                            &self.assets.sprites.fireball,
                        );
                        self.projectiles.push(fireball);
                    }
                }
            },
        }
    }
}
