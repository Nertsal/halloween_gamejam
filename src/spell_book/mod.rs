use geng::Camera2d;

use crate::{game::*, segment::Segment};

use super::*;

mod spell;

use spell::*;

pub struct SpellBook {
    geng: Geng,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,
    spell_grid: SpellGrid,
    spells: Vec<Spell>,
    player_cast: Option<SpellCast>,
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
            framebuffer_size: vec2(1.0, 1.0),
            spell_grid: SpellGrid::generate(2),
            spells: vec![
                Spell::new(
                    Command::Spawn {
                        spawn: CommandSpawn::Skeleton {
                            skeleton_type: SkeletonType::Archer,
                        },
                    },
                    vec![0, 3, 2, 1, 8, 7, 0, 1, 9],
                ),
                Spell::new(
                    Command::Spawn {
                        spawn: CommandSpawn::Skeleton {
                            skeleton_type: SkeletonType::Warrior,
                        },
                    },
                    vec![0, 4, 0, 8, 0, 6, 0, 2, 10],
                ),
                Spell::new(
                    Command::Spawn {
                        spawn: CommandSpawn::Fireball,
                    },
                    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                ),
            ],
            player_cast: None,
        }
    }

    fn finish_cast(&mut self) -> Option<&Command> {
        if let Some(spell_cast) = self.player_cast.take() {
            for spell in &self.spells {
                if spell.cast(&spell_cast) {
                    return Some(spell.command());
                }
            }
        }
        None
    }

    fn draw_spells(&self, framebuffer: &mut ugli::Framebuffer) {
        let mut next_position = Vec2::ZERO;
        let spell_grid = &self.spell_grid;
        for spell in &self.spells {
            let mut x_min = 0.0f32;
            let mut x_max = 0.0f32;
            let mut y_min = 0.0f32;
            let mut y_max = 0.0f32;

            for &point in &spell.key_points {
                let pos = spell_grid[point];
                x_min = x_min.min(pos.x);
                x_max = x_max.max(pos.x);
                y_min = y_min.min(pos.y);
                y_max = y_max.max(pos.y);
            }
            let spell_width = x_max - x_min;
            let y_center = (y_max + y_min) / 2.0;
            let offset = next_position - vec2(x_min, y_center);

            for index in 0..(spell.key_points.len() - 1) {
                let point = spell.key_points[index];
                let next = spell.key_points[index + 1];
                let start = spell_grid[point] + offset;
                let end = spell_grid[next] + offset;
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
                let point = spell.key_points[spell.key_points.len() - 1];
                let position = spell_grid[point] + offset;
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

    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        // ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);

        if let Some(spell_cast) = &self.player_cast {
            let offset = spell_cast.origin();

            for &point in self.spell_grid.iter() {
                self.geng.draw_2d().circle(
                    framebuffer,
                    &self.camera,
                    point + offset,
                    constants::SPELL_POINT_RADIUS,
                    Color::rgba(1.0, 1.0, 1.0, 0.2),
                );
            }

            for connection in spell_cast.connections() {
                let start = self.spell_grid[connection.from] + offset;
                let end = self.spell_grid[connection.to] + offset;
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
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        let _delta_time = delta_time as f32;
        if let Some(spell_cast) = &mut self.player_cast {
            let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
            let mouse_pos = self
                .camera
                .screen_to_world(self.framebuffer_size, mouse_pos);
            spell_cast.move_mouse(mouse_pos, &self.spell_grid);
        }
    }

    pub fn handle_event(&mut self, event: geng::Event) -> Option<Command> {
        match event {
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Left,
            } => {
                let mouse_pos = position.map(|x| x as f32);
                let mouse_pos = self
                    .camera
                    .screen_to_world(self.framebuffer_size, mouse_pos);
                self.player_cast = Some(SpellCast::new(mouse_pos));
            }
            geng::Event::MouseUp {
                button: geng::MouseButton::Left,
                ..
            } => {
                return self.finish_cast().cloned();
            }
            _ => (),
        }
        None
    }
}
