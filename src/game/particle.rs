use super::*;

pub struct Particle {
    pub circle: Circle,
    pub velocity: Vec2<f32>,
    pub color: Color<f32>,
}

impl Particle {
    pub fn new(circle: Circle, velocity: Vec2<f32>, color: Color<f32>) -> Self {
        Self {
            circle,
            velocity,
            color,
        }
    }
}

impl GameState {
    pub fn spawn_particles(
        &mut self,
        position: Vec2<f32>,
        radius: f32,
        speed: f32,
        color: Color<f32>,
        amount: usize,
    ) {
        self.particles.reserve(amount);
        for _ in 0..amount {
            let velocity = random_direction() * speed;
            let particle = Particle::new(Circle::new(position, radius), velocity, color);
            self.particles.push(particle);
        }
    }
}

pub fn random_direction() -> Vec2<f32> {
    let mut rng = global_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let (sin, cos) = angle.sin_cos();
    vec2(cos, sin)
}
