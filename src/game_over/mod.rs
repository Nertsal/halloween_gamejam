use geng::prelude::*;

use crate::Assets;

pub(crate) struct GameOverState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: geng::Camera2d,

    score: u32,
    best_score: AutoSave<u32>,
    transition: bool,
}

impl GameOverState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, score: u32) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: geng::Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 50.0,
            },

            score,
            best_score: {
                // Update best score
                let mut best_score = AutoSave::load("best_score.json");
                if score > *best_score {
                    *best_score = score;
                    best_score.save();
                }
                best_score
            },
            transition: false,
        }
    }
}

impl geng::State for GameOverState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let icon_pos = vec2(15.0, -2.5);
        let font = &self.assets.font;

        // Game over
        font.draw(
            framebuffer,
            &self.camera,
            "GAME OVER",
            vec2(0.0, 15.0),
            geng::TextAlign::CENTER,
            7.5,
            Color::WHITE,
        );

        // Score
        font.draw(
            framebuffer,
            &self.camera,
            &format!("SCORE: {}", self.score),
            vec2(-30.0, 0.0),
            geng::TextAlign::LEFT,
            7.5,
            Color::WHITE,
        );

        // Best Score
        font.draw(
            framebuffer,
            &self.camera,
            &format!("BEST:   {}", *self.best_score),
            vec2(-30.0, -10.0),
            geng::TextAlign::LEFT,
            7.5,
            Color::WHITE,
        );

        // PRESS ENTER TO STABILIZE
        font.draw(
            framebuffer,
            &self.camera,
            "PRESS ENTER TO RESSURECT",
            vec2(0.0, -20.0),
            geng::TextAlign::CENTER,
            5.0,
            Color::WHITE,
        );

        // Icon
        self.geng.draw_2d().textured_quad(
            framebuffer,
            &self.camera,
            AABB::point(icon_pos).extend_uniform(10.0),
            &self.assets.sprites.graves[2],
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

        Some(geng::Transition::Pop)
    }
}
