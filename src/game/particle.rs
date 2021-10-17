use super::*;

pub struct Particle {
    pub circle: Circle,
    pub velocity: Vec2<f32>,
    pub rotation: f32,
    pub rotation_velocity: f32,
    pub texture: ParticleTexture,
}

#[derive(Clone)]
pub enum ParticleTexture {
    Plain { color: Color<f32> },
    Textured { texture: Texture, alpha: f32 },
}

impl ParticleTexture {
    pub fn alpha(&self) -> f32 {
        match self {
            ParticleTexture::Plain { color } => color.a,
            ParticleTexture::Textured { alpha, .. } => *alpha,
        }
    }

    pub fn set_alpha(&mut self, new_alpha: f32) {
        match self {
            ParticleTexture::Plain { color } => color.a = new_alpha,
            ParticleTexture::Textured { alpha, .. } => *alpha = new_alpha,
        }
    }
}

impl Particle {
    pub fn new(
        circle: Circle,
        velocity: Vec2<f32>,
        rotation_velocity: f32,
        texture: ParticleTexture,
    ) -> Self {
        Self {
            circle,
            velocity,
            rotation_velocity,
            texture,
            rotation: 0.0,
        }
    }
}

impl GameState {
    pub fn spawn_particles(
        &mut self,
        position: Vec2<f32>,
        radius: f32,
        speed: f32,
        texture: ParticleTexture,
        amount: usize,
    ) {
        self.particles.reserve(amount);
        for _ in 0..amount {
            let velocity = random_direction() * speed;
            let particle = Particle::new(
                Circle::new(position, radius),
                velocity,
                global_rng().gen_range(-5.0..5.0),
                texture.clone(),
            );
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
