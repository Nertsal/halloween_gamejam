use geng::Camera2d;

use super::*;

mod constants;
mod player;
mod update;

use player::*;

pub(crate) struct GameState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
    player: Player,
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
            player: Player::new(
                Vec2::ZERO,
                constants::PLAYER_SPEED,
                constants::PLAYER_RADIUS,
                &assets.sprites.necromancer,
            ),
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(constants::BACKGROUND_COLOR), None);

        // Draw player
        self.geng.draw_2d().textured_quad(
            framebuffer,
            &self.camera,
            AABB::point(self.player.position).extend_uniform(self.player.radius),
            &self.player.texture,
            Color::WHITE,
        );

        // self.geng.draw_2d().textured_quad(
        //     framebuffer,
        //     &self.camera,
        //     AABB::point(self.player.position - vec2(5.0, 0.0)).extend_uniform(self.player.radius),
        //     &self.assets.sprites.skeleton,
        //     Color::WHITE,
        // );
        // self.geng.draw_2d().textured_quad(
        //     framebuffer,
        //     &self.camera,
        //     AABB::point(self.player.position + vec2(5.0, 0.0)).extend_uniform(self.player.radius),
        //     &self.assets.sprites.knight,
        //     Color::WHITE,
        // );
    }

    fn update(&mut self, delta_time: f64) {
        self.update(delta_time as f32);
    }
}
