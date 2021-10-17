use geng::prelude::*;

use crate::Assets;

pub(crate) struct MenuState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: geng::Camera2d,
    transition: bool,
}

impl MenuState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: geng::Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 50.0,
            },
            transition: false,
        }
    }
}

impl geng::State for MenuState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Ludum Dare 49 - Unstable
        let font = &self.assets.font;
        font.draw(
            framebuffer,
            &self.camera,
            "IGD Halloween Game Jam - Spooky Scary Skeletons",
            vec2(0.0, -24.5),
            geng::TextAlign::CENTER,
            2.0,
            Color::WHITE,
        );

        // Unstable Asteroids
        font.draw(
            framebuffer,
            &self.camera,
            "Skelemancer",
            vec2(0.0, 15.0),
            geng::TextAlign::CENTER,
            7.5,
            Color::WHITE,
        );

        // PRESS ENTER TO STABILIZE
        font.draw(
            framebuffer,
            &self.camera,
            "PRESS ENTER TO",
            vec2(-10.0, 0.0),
            geng::TextAlign::CENTER,
            5.0,
            Color::WHITE,
        );
        font.draw(
            framebuffer,
            &self.camera,
            "RESSURECT",
            vec2(-10.0, -10.0),
            geng::TextAlign::CENTER,
            5.0,
            Color::WHITE,
        );

        // Icon
        self.geng.draw_2d().textured_quad(
            framebuffer,
            &self.camera,
            AABB::point(vec2(20.0, 0.0)).extend_uniform(10.0),
            &self.assets.sprites.graves[0],
            Color::WHITE,
        );
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown {
                key: geng::Key::Enter,
            } => {
                self.transition = true;
                self.assets.sounds.select.play();
            }
            _ => (),
        }
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        if !self.transition {
            return None;
        }
        self.transition = false;

        let game_state = crate::game::GameState::new(&self.geng, &self.assets);
        Some(geng::Transition::Push(Box::new(game_state)))
    }
}
