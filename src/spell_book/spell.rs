use crate::renderable::Renderable;

use super::*;

pub struct Spell {
    command: Command,
    pub key_points: Vec<SpellPoint>,
}

pub struct SpellPoint {
    pub position: Vec2<f32>,
}

impl From<Vec2<f32>> for SpellPoint {
    fn from(position: Vec2<f32>) -> Self {
        Self::new(position)
    }
}

impl Spell {
    pub fn new<P>(command: Command, key_points: &[P]) -> Self
    where
        P: Copy + Into<SpellPoint>,
    {
        Self {
            command,
            key_points: key_points.iter().map(|&point| point.into()).collect(),
        }
    }

    pub fn command(self) -> Command {
        self.command
    }
}

impl SpellPoint {
    pub fn new(position: Vec2<f32>) -> Self {
        Self { position }
    }
}
