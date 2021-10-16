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
        for skeleton in &mut self.skeletons {
            skeleton.circle.position += skeleton.velocity.current * delta_time;
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

                knight.health.change(-constants::PLAYER_HIT_STRENGTH);
                player.health.change(-constants::KNIGHT_HIT_STRENGTH);
            }
        }

        // Knights - Skeletons
        for knight in &mut self.knights {
            for skeleton in &mut self.skeletons {
                if let Some(collision) = knight.circle.collision(&skeleton.circle) {
                    let shift = collision.normal * collision.penetration / 2.0;
                    knight.circle.position -= shift;
                    skeleton.circle.position += shift;

                    skeleton.velocity.current += collision.normal * constants::KNIGHT_HIT_FORCE;

                    knight.health.change(-constants::SKELETON_HIT_STRENGTH);
                    skeleton.health.change(-constants::KNIGHT_HIT_STRENGTH);
                }
            }
        }
    }

    fn kill(&mut self) {
        self.knights.retain(|knight| knight.health.is_alive());
        self.skeletons.retain(|skeleton| skeleton.health.is_alive());
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

        for skeleton in &mut self.skeletons {
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
                    skeleton.velocity.accelerate(delta_time);
                }
            }
        }

        for (position, radius, speed, color, amount) in particles {
            self.spawn_particles(position, radius, speed, color, amount);
        }
    }

    fn update_knights(&mut self, delta_time: f32) {
        for knight in &mut self.knights {
            // Find target
            let targets = self
                .skeletons
                .iter()
                .map(|skeleton| skeleton.circle.position);
            let targets = targets.chain(std::iter::once(self.player.circle.position));
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
