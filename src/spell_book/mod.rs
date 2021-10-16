use geng::Camera2d;

use crate::game::*;

use super::*;

mod segment;
mod spell;

use segment::*;
use spell::*;

pub struct SpellBook {
    geng: Geng,
    camera: Camera2d,
    spells: Vec<Spell>,
}

impl SpellBook {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            spells: vec![Spell::new(
                Command::Spawn {
                    position: Vec2::ZERO,
                    spawn: CommandSpawn::Skeleton {
                        skeleton_type: SkeletonType::Archer,
                    },
                },
                &[
                    vec2(10.0, 0.0),
                    vec2(-3.0, 0.0),
                    vec2(0.0, -10.0),
                    vec2(4.0, -5.0),
                    vec2(6.0, 0.0),
                    vec2(4.0, 5.0),
                    vec2(0.0, 10.0),
                    vec2(-3.0, 0.0),
                ],
            )],
        }
    }
}

impl geng::State for SpellBook {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        // ugli::clear(framebuffer, Some(Color::BLACK), None);

        let mut next_position = Vec2::ZERO;
        for spell in &self.spells {
            let mut x_min = 0.0f32;
            let mut x_max = 0.0f32;
            let mut y_min = 0.0f32;
            let mut y_max = 0.0f32;

            for point in &spell.key_points {
                x_min = x_min.min(point.position.x);
                x_max = x_max.max(point.position.x);
                y_min = y_min.min(point.position.y);
                y_max = y_max.max(point.position.y);
            }
            let spell_width = x_max - x_min;
            let y_center = (y_max + y_min) / 2.0;
            let offset = next_position - vec2(x_min, y_center);

            for index in 0..(spell.key_points.len() - 1) {
                let point = &spell.key_points[index];
                let next = &spell.key_points[index + 1];
                let start = point.position + offset;
                let end = next.position + offset;
                self.geng.draw_2d().circle(
                    framebuffer,
                    &self.camera,
                    start,
                    constants::SPELL_POINT_RADIUS,
                    Color::WHITE,
                );
                self.geng.draw_2d().draw(
                    framebuffer,
                    &self.camera,
                    &Segment {
                        start,
                        end,
                        width: constants::SPELL_CONNECTION_WIDTH,
                    }
                    .polygon(),
                    Color::WHITE,
                    ugli::DrawMode::TriangleFan,
                );
            }
            if spell.key_points.len() > 0 {
                let point = &spell.key_points[spell.key_points.len() - 1];
                let position = point.position + offset;
                self.geng.draw_2d().circle(
                    framebuffer,
                    &self.camera,
                    position,
                    constants::SPELL_POINT_RADIUS,
                    Color::WHITE,
                );
            }

            next_position += vec2(constants::SPELLS_MARGIN + spell_width, 0.0);
        }
    }
}
