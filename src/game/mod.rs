use geng::Camera2d;

use super::*;

mod constants;
mod knight;
mod particle;
mod player;
mod skeleton;
mod update;

use knight::*;
use particle::*;
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
                Vec2::ZERO,
                constants::PLAYER_SPEED,
                constants::PLAYER_RADIUS,
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
                particle.position,
                particle.radius,
                particle.color,
            );
        }

        // Draw skeletons
        for skeleton in &self.skeletons {
            self.geng.draw_2d().textured_quad(
                framebuffer,
                &self.camera,
                AABB::point(skeleton.position).extend_uniform(skeleton.radius),
                &skeleton.texture,
                Color::WHITE,
            );
        }

        // Draw knights
        for knight in &self.knights {
            self.geng.draw_2d().textured_quad(
                framebuffer,
                &self.camera,
                AABB::point(knight.position).extend_uniform(knight.radius),
                &knight.texture,
                Color::WHITE,
            );
        }

        // Draw player
        self.geng.draw_2d().textured_quad(
            framebuffer,
            &self.camera,
            AABB::point(self.player.position).extend_uniform(self.player.radius),
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
