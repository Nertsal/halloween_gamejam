use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        self.difficulty.update(delta_time);

        if let Some((spawn_timer, spawns_left)) = &mut self.spawn_timer {
            let mut spawn_knight = false;
            *spawn_timer -= delta_time;
            if *spawn_timer <= 0.0 {
                *spawn_timer = constants::SPAWN_DELAY;
                spawn_knight = true;
                *spawns_left -= 1;
                if *spawns_left == 0 {
                    self.spawn_timer = None;
                }
            }
            if spawn_knight {
                self.spawn_knight(self.castle.bottom());
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
                }
            }
            for projectile in &mut self.projectiles {
                if let Some(collision) = knight.circle.collision(&projectile.circle) {
                    knight.velocity.current -= collision.normal * constants::ARROW_HIT_FORCE;

                    projectile.hit = true;
                    knight.health.change(-match projectile.typ {
                        ProjectileType::Arrow => constants::ARROW_HIT_DAMAGE,
                        ProjectileType::Fireball => constants::FIREBALL_HIT_DAMAGE,
                    });
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
        let mut particles = Vec::new();
        let knights = &mut self.knights;
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
                        particles.push((
                            projectile.circle.position,
                            0.25,
                            10.0,
                            Color::rgba(0.7, 0.1, 0.1, constants::PARTICLE_ALPHA),
                            50,
                        ));
                    }
                }
            }
            !hit
        });

        let player = &mut self.player;
        self.knights.retain(|knight| {
            let alive = knight.health.is_alive();
            if !alive {
                player.mana.change(constants::KNIGHT_KILL_MANA);
            }
            alive
        });

        self.skeletons_warriors
            .retain(|skeleton| skeleton.health.is_alive());
        self.skeletons_archers
            .retain(|skeleton| skeleton.health.is_alive());

        for (position, radius, speed, color, amount) in particles {
            self.spawn_particles(position, radius, speed, color, amount);
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

        let move_direction = vec2(move_x, move_y).clamp(1.0);
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
                    let targets = self.knights.iter().map(|knight| knight.circle.position);
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

                    let targets = self.knights.iter().map(|knight| {
                        (
                            knight,
                            (knight.circle.position - skeleton.circle.position).len(),
                        )
                    });
                    let target = targets.min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
                    let target = if target.map(|(_, distance)| distance).unwrap_or_default() <= 1e-5
                    {
                        skeleton.circle.position
                    } else {
                        let (target, distance) = target.unwrap();
                        if skeleton.shoot_timer <= 0.0 {
                            // let time = distance / constants::ARROW_SPEED;
                            let prediction = target.circle.position; // + target.velocity.current * time;
                            let direction = (prediction - skeleton.circle.position).normalize();
                            direction.rotate(rng.gen_range(
                                -constants::SKELETON_ARCHER_RANDOMNESS
                                    ..constants::SKELETON_ARCHER_RANDOMNESS,
                            ));
                            projectiles.push((skeleton.circle.position, direction));
                            skeleton.shoot_timer = skeleton.shoot_cooldown;
                        }

                        let target = target.circle.position;
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
            self.spawn_particles(position, radius, speed, color, amount);
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
    }

    fn update_particles(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.color.a -= delta_time * constants::PARTICLE_DECAY_SPEED;
        }

        // Delete old particles
        self.particles.retain(|particle| particle.color.a > 0.0);
    }
}
