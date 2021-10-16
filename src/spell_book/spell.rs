use super::*;

pub struct SpellGrid {
    grid: Vec<Vec2<f32>>,
}

impl SpellGrid {
    pub fn generate(rings: usize) -> Self {
        let mut grid = Vec::with_capacity(1 + rings * 8);
        grid.push(vec2(0.0, 0.0));

        for ring in 0..rings {
            for i in 0..8 {
                let angle = i as f32 / 8.0 * std::f32::consts::PI * 2.0;
                let (sin, cos) = angle.sin_cos();
                let direction = vec2(cos, sin);
                grid.push(direction * (ring + 1) as f32 * constants::SPELL_RING_RADIUS);
            }
        }

        Self { grid }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec2<f32>> {
        self.grid.iter()
    }

    fn point(&self, position: Vec2<f32>) -> Option<usize> {
        self.grid
            .iter()
            .enumerate()
            .find(|(_, &point)| (position - point).len() <= constants::SPELL_POINT_RADIUS)
            .map(|(index, _)| index)
    }
}

impl Index<usize> for SpellGrid {
    type Output = Vec2<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

pub struct Spell {
    command: Command,
    pub key_points: Vec<usize>,
}

impl Spell {
    pub fn new(command: Command, key_points: Vec<usize>) -> Self {
        Self {
            command,
            key_points,
        }
    }

    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn cast(&self, cast: &SpellCast) -> bool {
        let mut con1 = self.connections();
        con1.sort();
        con1.dedup();

        let mut con2 = cast.connections();
        con2.sort();
        con2.dedup();

        con1 == con2
    }

    fn connections(&self) -> Vec<Connection<usize>> {
        vec_to_connections(&self.key_points)
    }
}

pub struct SpellCast {
    initial_mouse_pos: Vec2<f32>,
    key_points: Vec<usize>,
}

impl SpellCast {
    pub fn new(mouse_pos: Vec2<f32>) -> Self {
        Self {
            initial_mouse_pos: mouse_pos,
            key_points: vec![],
        }
    }

    pub fn origin(&self) -> Vec2<f32> {
        self.initial_mouse_pos
    }

    pub fn connections(&self) -> Vec<Connection<usize>> {
        vec_to_connections(&self.key_points)
    }

    pub fn move_mouse(&mut self, mouse_pos: Vec2<f32>, spell_grid: &SpellGrid) {
        if let Some(point) = spell_grid.point(mouse_pos - self.initial_mouse_pos) {
            if Some(&point) != self.key_points.last() {
                self.key_points.push(point);
            }
        }
    }
}

#[derive(PartialOrd, Ord)]
pub struct Connection<T> {
    pub from: T,
    pub to: T,
}

impl<T: PartialEq> PartialEq for Connection<T> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
            || self.from == other.to && self.to == other.from
    }
}
impl<T: Eq> Eq for Connection<T> {}

fn vec_to_connections<T: Copy>(vec: &Vec<T>) -> Vec<Connection<T>> {
    let len = vec.len();
    if len == 0 {
        return vec![];
    }

    let mut connections = Vec::with_capacity(len - 1);
    for i in 0..len - 1 {
        let from = vec[i];
        let to = vec[i + 1];
        connections.push(Connection { from, to });
    }
    connections
}
