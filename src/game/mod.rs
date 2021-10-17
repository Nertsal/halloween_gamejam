use geng::Camera2d;

use crate::spell_book::SpellBook;

use super::*;

mod command;
mod difficulty;
mod draw;
mod health;
mod knight;
mod particle;
mod physics;
mod player;
mod projectile;
mod rogue;
mod skeleton;
mod textured_circle;
mod update;
mod velocity;

pub use command::*;
pub use skeleton::SkeletonType;

use difficulty::*;
use health::*;
use knight::*;
use particle::*;
use physics::*;
use player::*;
use projectile::*;
use rogue::*;
use skeleton::*;
use textured_circle::*;
use velocity::*;

pub(crate) struct GameState {
    // Engine stuff
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,

    // Gameplay
    bounds: AABB<f32>,
    difficulty: Difficulty,
    score: u32,
    spawn_timer: Option<(f32, usize)>,
    player: Player,
    skeletons_warriors: Vec<SkeletonWarrior>,
    skeletons_archers: Vec<SkeletonArcher>,
    projectiles: Vec<Projectile>,
    knights: Vec<Knight>,
    rogues: Vec<Rogue>,
    particles: Vec<Particle>,

    // Cosmetic
    castle: TexturedCircle,
    graves: Vec<TexturedCircle>,

    // SpellBook
    is_spell_book_open: bool,
    spell_book: SpellBook,
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

            bounds: AABB::point(vec2(constants::ARENA_CENTER_X, constants::ARENA_CENTER_Y))
                .extend_symmetric(vec2(constants::ARENA_WIDTH, constants::ARENA_HEIGHT) / 2.0),
            difficulty: Difficulty::new(),
            score: 0,
            spawn_timer: None,
            player: Player::new(
                Circle::new(Vec2::ZERO, constants::PLAYER_RADIUS),
                constants::PLAYER_SPEED,
                Health::new(constants::PLAYER_HEALTH),
                Mana::new(constants::PLAYER_MANA),
                &assets.sprites.necromancer,
            ),
            skeletons_warriors: vec![],
            skeletons_archers: vec![],
            projectiles: vec![],
            knights: vec![],
            rogues: vec![],
            particles: vec![],

            castle: TexturedCircle::new(
                Circle::new(
                    vec2(0.0, 47.5 - constants::CASTLE_SIZE),
                    constants::CASTLE_SIZE,
                ),
                &assets.sprites.castle,
            ),
            graves: {
                let mut graves = Vec::with_capacity(8);
                let mut rng = global_rng();
                for x in 0..2 {
                    for y in 0..3 {
                        let position = vec2((x * 2 - 1) as f32 * 20.0, 0.0 - y as f32 * 15.0);
                        let offset = vec2(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..2.0));
                        let position = position + offset;
                        let texture = assets.sprites.graves.choose(&mut rng).unwrap();
                        let grave = TexturedCircle::new(
                            Circle::new(position, constants::GRAVE_SIZE),
                            texture,
                        );
                        graves.push(grave);
                    }
                }
                graves
            },

            is_spell_book_open: true,
            spell_book: SpellBook::new(geng),
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw_impl(framebuffer);
        if self.is_spell_book_open {
            self.spell_book.draw(framebuffer);
        }
    }

    fn update(&mut self, delta_time: f64) {
        self.update_impl(delta_time as f32);
        self.spell_book.update(delta_time);
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown { button, .. } => match button {
                geng::MouseButton::Right => self.spawn_knight(self.castle.bottom()),
                _ => (),
            },
            _ => (),
        }
        if let Some(command) = self.spell_book.handle_event(event) {
            self.perform_command(command);
        }
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        if self.player.health.is_alive() {
            return None;
        }

        Some(geng::Transition::Switch(Box::new(
            game_over::GameOverState::new(&self.geng, &self.assets, self.score),
        )))
    }
}
