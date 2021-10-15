use geng::Camera2d;

use super::*;

mod constants;
mod health;
mod knight;
mod particle;
mod physics;
mod player;
mod skeleton;
mod update;

use health::*;
use knight::*;
use particle::*;
use physics::*;
use player::*;
use skeleton::*;

pub(crate) struct GameState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,

    player: Player,
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
                Circle::new(Vec2::ZERO, constants::PLAYER_SPEED),
                constants::PLAYER_RADIUS,
                Health::new(constants::PLAYER_HEALTH),
                &assets.sprites.necromancer,
            ),
            skeletons: vec![],
            knights: vec![],
            particles: vec![],
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(constants::BACKGROUND_COLOR), None);

        // Draw particles
        for particle in &self.particles {
            self.geng.draw_2d().circle(
                framebuffer,
                &self.camera,
                particle.circle.position,
                particle.circle.radius,
                particle.color,
            );
        }

        // Draw skeletons
        for skeleton in &self.skeletons {
            self.geng.draw_2d().textured_quad(
                framebuffer,
                &self.camera,
                AABB::point(skeleton.circle.position).extend_uniform(skeleton.circle.radius),
                &skeleton.texture,
                Color::WHITE,
            );
        }

        // Draw knights
        for knight in &self.knights {
            self.geng.draw_2d().textured_quad(
                framebuffer,
                &self.camera,
                AABB::point(knight.circle.position).extend_uniform(knight.circle.radius),
                &knight.texture,
                Color::WHITE,
            );
        }

        // Draw player
        self.geng.draw_2d().textured_quad(
            framebuffer,
            &self.camera,
            AABB::point(self.player.circle.position).extend_uniform(self.player.circle.radius),
            &self.player.texture,
            Color::WHITE,
        );
    }

    fn update(&mut self, delta_time: f64) {
        self.update(delta_time as f32);
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown { position, button } => {
                let position = position.map(|x| x as f32);
                let position = self.camera.screen_to_world(self.framebuffer_size, position);
                match button {
                    geng::MouseButton::Left => self.spawn_skeleton(position),
                    geng::MouseButton::Right => self.spawn_knight(position),
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
