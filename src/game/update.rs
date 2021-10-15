use super::*;

impl GameState {
    pub fn update(&mut self, delta_time: f32) {
        self.update_player();
        self.update_skeletons(delta_time);
        self.update_particles(delta_time);
        self.update_knights(delta_time);

        self.movement(delta_time);
    }

    fn movement(&mut self, delta_time: f32) {
        // Player
        self.player.position += self.player.velocity * delta_time;

        // Skeletons
        for skeleton in &mut self.skeletons {
            skeleton.position += skeleton.velocity * delta_time;
        }

        // Knigths
        for knight in &mut self.knights {
            knight.position += knight.velocity * delta_time;
        }

        // Particles
        for particle in &mut self.particles {
            particle.position += particle.velocity * delta_time;
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
                            skeleton.position,
                            0.25,
                            3.0,
                            Color::rgba(0.5, 0.5, 0.5, constants::PARTICLE_ALPHA),
                            50,
                        ));
                    }
                }
                SkeletonState::Alive => (),
            }
        }

        for (position, radius, speed, color, amount) in particles {
            self.spawn_particles(position, radius, speed, color, amount);
        }
    }

    fn update_knights(&mut self, delta_time: f32) {}

    fn update_particles(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.color.a -= delta_time * constants::PARTICLE_DECAY_SPEED;
        }

        // Delete old particles
        self.particles.retain(|particle| particle.color.a > 0.0);
    }
}
