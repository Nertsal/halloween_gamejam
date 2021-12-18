use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        self.difficulty.update(delta_time);

        if let Some((spawn_timer, spawns_left)) = &mut self.spawn_timer {
            let mut spawn_enemy = false;
            *spawn_timer -= delta_time;
            if *spawn_timer <= 0.0 {
                *spawn_timer = constants::SPAWN_DELAY;
                spawn_enemy = true;
                *spawns_left -= 1;
                if *spawns_left == 0 {
                    self.spawn_timer = None;
                }
            }
            if spawn_enemy {
                if global_rng().gen_bool(0.5) {
                    self.spawn_knight(self.castle.bottom());
                } else {
                    self.spawn_rogue(self.castle.bottom());
                }
            }
        } else {
            if self.knights.len() == 0 {
                self.difficulty.next_stage();
                self.spawn_timer =
                    Some((constants::FIRST_SPAWN_DELAY, self.difficulty.spawn_count()))
            }
        }

        self.update_player();
        self.update_skeletons(delta_time);
        self.update_particles(delta_time);
        self.update_knights(delta_time);

        self.movement(delta_time);

        self.collision();

        self.kill();
    }

    fn movement(&mut self, delta_time: f32) {
        // Player
        self.player.circle.position += self.player.velocity * delta_time;

        // Skeletons
        for skeleton in &mut self.skeletons_warriors {
            skeleton.circle.position += skeleton.velocity.current * delta_time;
        }
        for skeleton in &mut self.skeletons_archers {
            skeleton.circle.position += skeleton.velocity.current * delta_time;
        }

        // Projectiles
        for projectile in &mut self.projectiles {
            projectile.circle.position += projectile.velocity * delta_time;
        }

        // Knights
        for knight in &mut self.knights {
            knight.circle.position += knight.velocity.current * delta_time;
        }
        // Rogues
        for rogue in &mut self.rogues {
            rogue.circle.position += rogue.velocity.current * delta_time;
        }

        // Particles
        for particle in &mut self.particles {
            particle.circle.position += particle.velocity * delta_time;
        }
    }

    fn collision(&mut self) {
        // Player - Knights
        let player = &mut self.player;
        for knight in &mut self.knights {
            if let Some(collision) = player.circle.collision(&knight.circle) {
                knight.circle.position += collision.normal * collision.penetration;
                knight.velocity.current += collision.normal * constants::PLAYER_HIT_FORCE;

                player.health.change(-constants::KNIGHT_HIT_DAMAGE);
                knight.health.change(-constants::PLAYER_HIT_DAMAGE);
                self.assets.sounds.hit.play();
            }
        }

        // Player - Rogues
        let player = &mut self.player;
        for rogue in &mut self.rogues {
            if let Some(collision) = player.circle.collision(&rogue.circle) {
                rogue.circle.position += collision.normal * collision.penetration;
                rogue.velocity.current += collision.normal * constants::PLAYER_HIT_FORCE;

                player.health.change(-constants::ROGUE_HIT_DAMAGE);
                rogue.health.change(-constants::PLAYER_HIT_DAMAGE);
                self.assets.sounds.hit.play();
            }
        }

        // Knights - Skeletons, Projectiles
        for knight in &mut self.knights {
            for skeleton in &mut self.skeletons_warriors {
                if let Some(collision) = knight.circle.collision(&skeleton.circle) {
                    let shift = collision.normal * collision.penetration / 2.0;
                    knight.circle.position -= shift;
                    skeleton.circle.position += shift;

                    skeleton.velocity.current += collision.normal * constants::KNIGHT_HIT_FORCE;
                    knight.velocity.current -=
                        collision.normal * constants::SKELETON_WARRIOR_HIT_FORCE;

                    skeleton.health.change(-constants::KNIGHT_HIT_DAMAGE);
                    knight.health.change(-constants::SKELETON_HIT_DAMAGE);
                    self.assets.sounds.hit.play();
                }
            }
            for skeleton in &mut self.skeletons_archers {
                if let Some(collision) = knight.circle.collision(&skeleton.circle) {
                    let shift = collision.normal * collision.penetration / 2.0;
                    knight.circle.position -= shift;
                    skeleton.circle.position += shift;

                    skeleton.velocity.current += collision.normal * constants::KNIGHT_HIT_FORCE;

                    skeleton.health.change(-constants::KNIGHT_HIT_DAMAGE);
                    knight.health.change(-constants::SKELETON_HIT_DAMAGE);
                    self.assets.sounds.hit.play();
                }
            }
            for projectile in &mut self.projectiles {
                if let Some(collision) = knight.circle.collision(&projectile.circle) {
                    knight.velocity.current -= collision.normal * constants::ARROW_HIT_FORCE;

                    projectile.hit = true;
                    knight.health.change(-match projectile.typ {
                        ProjectileType::Arrow => (constants::ARROW_HIT_DAMAGE
                            - constants::KNIGHT_ARROW_RESISTANCE)
                            .max(0.0),
                        ProjectileType::Fireball => constants::FIREBALL_HIT_DAMAGE,
                    });
                    self.assets.sounds.hit.play();
                }
            }
        }

        // Rogues - Skeletons, Projectiles
        for rogue in &mut self.rogues {
            for skeleton in &mut self.skeletons_warriors {
                if let Some(collision) = rogue.circle.collision(&skeleton.circle) {
                    let shift = collision.normal * collision.penetration / 2.0;
                    rogue.circle.position -= shift;
                    skeleton.circle.position += shift;

                    skeleton.velocity.current += collision.normal * constants::ROGUE_HIT_FORCE;
                    rogue.velocity.current -=
                        collision.normal * constants::SKELETON_WARRIOR_HIT_FORCE;

                    skeleton.health.change(-constants::KNIGHT_HIT_DAMAGE);
                    rogue.health.change(-constants::SKELETON_HIT_DAMAGE);
                    self.assets.sounds.hit.play();
                }
            }
            for skeleton in &mut self.skeletons_archers {
                if let Some(collision) = rogue.circle.collision(&skeleton.circle) {
                    let shift = collision.normal * collision.penetration / 2.0;
                    rogue.circle.position -= shift;
                    skeleton.circle.position += shift;

                    skeleton.velocity.current += collision.normal * constants::ROGUE_HIT_FORCE;

                    skeleton.health.change(-constants::KNIGHT_HIT_DAMAGE);
                    rogue.health.change(-constants::SKELETON_HIT_DAMAGE);
                    self.assets.sounds.hit.play();
                }
            }
            for projectile in &mut self.projectiles {
                if let Some(collision) = rogue.circle.collision(&projectile.circle) {
                    rogue.velocity.current -= collision.normal * constants::ARROW_HIT_FORCE;

                    projectile.hit = true;
                    rogue.health.change(-match projectile.typ {
                        ProjectileType::Arrow => constants::ARROW_HIT_DAMAGE,
                        ProjectileType::Fireball => constants::FIREBALL_HIT_DAMAGE,
                    });
                    self.assets.sounds.hit.play();
                }
            }
        }

        // Player - Border
        self.player.circle.bounds(&self.bounds);

        // Skeletons - Border
        for skeleton in &mut self.skeletons_warriors {
            skeleton.circle.bounds(&self.bounds);
        }
        for skeleton in &mut self.skeletons_archers {
            skeleton.circle.bounds(&self.bounds);
        }
    }

    fn kill(&mut self) {
        let mut new_particles = Vec::new();
        let knights = &mut self.knights;
        let rogues = &mut self.rogues;
        self.projectiles.retain(|projectile| {
            let hit = projectile.hit;
            if hit {
                match projectile.typ {
                    ProjectileType::Arrow => (),
                    ProjectileType::Fireball => {
                        for knight in knights.iter_mut() {
                            let distance =
                                (knight.circle.position - projectile.circle.position).len();
                            if distance <= constants::FIREBALL_EXPLOSION_RADIUS {
                                knight.health.change(-constants::FIREBALL_EXPLOSION_DAMAGE);
                            }
                        }
                        for rogue in rogues.iter_mut() {
                            let distance =
                                (rogue.circle.position - projectile.circle.position).len();
                            if distance <= constants::FIREBALL_EXPLOSION_RADIUS {
                                rogue.health.change(-constants::FIREBALL_EXPLOSION_DAMAGE);
                            }
                        }
                        new_particles.push((
                            projectile.circle.position,
                            0.25,
                            10.0,
                            ParticleTexture::Plain {
                                color: Color::rgba(0.7, 0.1, 0.1, constants::PARTICLE_ALPHA),
                            },
                            50,
                        ));
                    }
                }
            }
            !hit
        });

        let player = &mut self.player;
        let assets = &self.assets;
        let score = &mut self.score;
        self.knights.retain(|knight| {
            let alive = knight.health.is_alive();
            if !alive {
                *score += 1;
                player.mana.change(constants::KNIGHT_KILL_MANA);
                new_particles.push((
                    knight.circle.position,
                    knight.circle.radius,
                    0.0,
                    ParticleTexture::Textured {
                        texture: assets.sprites.dead_knight.clone(),
                        alpha: constants::PARTICLE_ALPHA,
                    },
                    1,
                ));
            }
            alive
        });
        self.rogues.retain(|rogue| {
            let alive = rogue.health.is_alive();
            if !alive {
                player.mana.change(constants::KNIGHT_KILL_MANA);
                new_particles.push((
                    rogue.circle.position,
                    rogue.circle.radius,
                    0.0,
                    ParticleTexture::Textured {
                        texture: assets.sprites.dead_rogue.clone(),
                        alpha: constants::PARTICLE_ALPHA,
                    },
                    1,
                ));
            }
            alive
        });

        self.skeletons_warriors.retain(|skeleton| {
            let alive = skeleton.health.is_alive();
            if !alive {
                new_particles.push((
                    skeleton.circle.position,
                    1.0,
                    20.0,
                    ParticleTexture::Textured {
                        texture: assets.sprites.bone.clone(),
                        alpha: constants::PARTICLE_ALPHA,
                    },
                    5,
                ));
            }
            alive
        });
        self.skeletons_archers.retain(|skeleton| {
            let alive = skeleton.health.is_alive();
            if !alive {
                new_particles.push((
                    skeleton.circle.position,
                    1.0,
                    20.0,
                    ParticleTexture::Textured {
                        texture: assets.sprites.bone.clone(),
                        alpha: constants::PARTICLE_ALPHA,
                    },
                    5,
                ));
            }
            alive
        });

        for (position, radius, speed, texture, amount) in new_particles {
            self.spawn_particles(position, radius, speed, texture, amount);
        }
    }

    fn update_player(&mut self) {
        let mut move_x = 0.0;
        let mut move_y = 0.0;
        let window = self.geng.window();
        use geng::Key;
        if window.is_key_pressed(Key::W) {
            move_y += 1.0;
        }
        if window.is_key_pressed(Key::S) {
            move_y += -1.0;
        }
        if window.is_key_pressed(Key::A) {
            move_x += -1.0;
        }
        if window.is_key_pressed(Key::D) {
            move_x += 1.0;
        }

        if move_x > 0.0 {
            self.player.sprite.flipped = false;
        } else if move_x < 0.0 {
            self.player.sprite.flipped = true;
        }

        let move_direction = vec2(move_x, move_y).clamp_len(..=1.0);
        self.player.velocity = self.player.speed * move_direction;
    }

    fn update_skeletons(&mut self, delta_time: f32) {
        let mut particles = Vec::new();
        let mut rng = global_rng();

        for skeleton in &mut self.skeletons_warriors {
            match &mut skeleton.state {
                SkeletonState::Spawning { time_left } => {
                    *time_left -= delta_time;
                    if *time_left <= 0.0 {
                        skeleton.state = SkeletonState::Alive;
                        particles.push((
                            skeleton.circle.position,
                            0.25,
                            3.0,
                            Color::rgba(0.5, 0.5, 0.5, constants::PARTICLE_ALPHA),
                            50,
                        ));
                    }
                }
                SkeletonState::Alive => {
                    // Find the target
                    let targets = self
                        .knights
                        .iter()
                        .map(|knight| knight.circle.position)
                        .chain(self.rogues.iter().map(|rogue| rogue.circle.position));
                    let targets = targets
                        .map(|position| (position, (position - skeleton.circle.position).len()));
                    let (target, _) = targets
                        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                        .unwrap_or((skeleton.circle.position, 0.0));
                    skeleton.target(target);

                    skeleton.velocity.accelerate(delta_time);
                }
            }
        }

        let mut projectiles = Vec::new();
        for skeleton in &mut self.skeletons_archers {
            match &mut skeleton.state {
                SkeletonState::Spawning { time_left } => {
                    *time_left -= delta_time;
                    if *time_left <= 0.0 {
                        skeleton.state = SkeletonState::Alive;
                        particles.push((
                            skeleton.circle.position,
                            0.25,
                            3.0,
                            Color::rgba(0.5, 0.5, 0.5, constants::PARTICLE_ALPHA),
                            50,
                        ));
                    }
                }
                SkeletonState::Alive => {
                    // Find the target
                    if skeleton.shoot_timer > 0.0 {
                        skeleton.shoot_timer -= delta_time;
                    }

                    let targets = self
                        .knights
                        .iter()
                        .map(|knight| knight.circle.position)
                        .chain(self.rogues.iter().map(|rogue| (rogue.circle.position)))
                        .map(|position| (position, (position - skeleton.circle.position).len()));
                    let target = targets.min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
                    let target = if target.map(|(_, distance)| distance).unwrap_or_default() <= 1e-5
                    {
                        skeleton.circle.position
                    } else {
                        let (target, distance) = target.unwrap();
                        if skeleton.shoot_timer <= 0.0 {
                            // let time = distance / constants::ARROW_SPEED;
                            let prediction = target; // + target.velocity.current * time;
                            let direction = (prediction - skeleton.circle.position).normalize();
                            direction.rotate(rng.gen_range(
                                -constants::SKELETON_ARCHER_RANDOMNESS
                                    ..constants::SKELETON_ARCHER_RANDOMNESS,
                            ));
                            projectiles.push((skeleton.circle.position, direction));
                            skeleton.shoot_timer = skeleton.shoot_cooldown;
                        }

                        let target = target;
                        let direction = (skeleton.circle.position - target) / distance;
                        direction * constants::SKELETON_ARCHER_DISTANCE + target
                    };
                    skeleton.target(target);
                    skeleton.velocity.accelerate(delta_time);
                }
            }
        }

        if !projectiles.is_empty() {
            self.assets.sounds.shoot.play();
        }

        for (position, direction) in projectiles {
            self.projectiles.push(Projectile::new(
                Circle::new(position, constants::ARROW_RADIUS),
                direction * constants::ARROW_SPEED,
                &self.assets.sprites.arrow,
                ProjectileType::Arrow,
            ));
        }

        for (position, radius, speed, color, amount) in particles {
            self.spawn_particles(
                position,
                radius,
                speed,
                ParticleTexture::Plain { color },
                amount,
            );
        }
    }

    fn update_knights(&mut self, delta_time: f32) {
        for knight in &mut self.knights {
            // Find target
            let targets = self
                .skeletons_warriors
                .iter()
                .map(|skeleton| skeleton.circle.position)
                .chain(
                    self.skeletons_archers
                        .iter()
                        .map(|skeleton| skeleton.circle.position),
                )
                .chain(std::iter::once(self.player.circle.position));
            let targets =
                targets.map(|position| (position, (position - knight.circle.position).len()));
            let target = targets
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .expect("Is player dead?");
            knight.target(target.0);

            // Accelerate
            knight.velocity.accelerate(delta_time);
        }

        for rogue in &mut self.rogues {
            // Find target
            let targets = self
                .skeletons_warriors
                .iter()
                .map(|skeleton| skeleton.circle.position)
                .chain(
                    self.skeletons_archers
                        .iter()
                        .map(|skeleton| skeleton.circle.position),
                )
                .chain(std::iter::once(self.player.circle.position));
            let targets =
                targets.map(|position| (position, (position - rogue.circle.position).len()));
            let target = targets
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .expect("Is player dead?");
            rogue.target(target.0);

            // Accelerate
            rogue.velocity.accelerate(delta_time);
        }
    }

    fn update_particles(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            let alpha = particle.texture.alpha();
            particle
                .texture
                .set_alpha(alpha - delta_time * constants::PARTICLE_DECAY_SPEED);

            particle.rotation += particle.rotation_velocity * delta_time;
        }

        // Delete old particles
        self.particles
            .retain(|particle| particle.texture.alpha() > 0.0);
    }
}
