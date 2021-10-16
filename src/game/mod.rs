use geng::Camera2d;

use super::*;

mod castle;
mod constants;
mod draw;
mod health;
mod knight;
mod particle;
mod physics;
mod player;
mod skeleton;
mod update;
mod velocity;

use castle::*;
use health::*;
use knight::*;
use particle::*;
use physics::*;
use player::*;
use skeleton::*;
use velocity::*;

pub(crate) struct GameState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,

    player: Player,
    castle: Castle,
    skeletons: Vec<Skeleton>,
    knights: Vec<Knight>,
    particles: Vec<Particle>,
}

impl GameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            framebuffer_size: vec2(1.0, 1.0),

            player: Player::new(
                Circle::new(Vec2::ZERO, constants::PLAYER_RADIUS),
                constants::PLAYER_SPEED,
                Health::new(constants::PLAYER_HEALTH),
                &assets.sprites.necromancer,
            ),
            castle: Castle::new(
                Circle::new(
                    vec2(0.0, 47.5 - constants::CASTLE_SIZE),
                    constants::CASTLE_SIZE,
                ),
                &assets.sprites.castle,
            ),
            skeletons: vec![],
            knights: vec![],
            particles: vec![],
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw_impl(framebuffer);
    }

    fn update(&mut self, delta_time: f64) {
        self.update_impl(delta_time as f32);
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown { position, button } => {
                let position = position.map(|x| x as f32);
                let position = self.camera.screen_to_world(self.framebuffer_size, position);
                match button {
                    geng::MouseButton::Left => self.spawn_skeleton(position),
                    geng::MouseButton::Right => self.spawn_knight(self.castle.spawn_position()),
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
