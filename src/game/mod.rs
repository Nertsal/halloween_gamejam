use geng::Camera2d;

use super::*;

pub struct GameState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
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
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.geng.draw_2d().textured_quad(
            framebuffer,
            &self.camera,
            AABB::ZERO.extend_uniform(5.0),
            &self.assets.sprites.skeleton,
            Color::WHITE,
        );
    }
}
