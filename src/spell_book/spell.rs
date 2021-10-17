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
        let con1 = normalize_connections(self.connections());
        let con2 = normalize_connections(cast.connections());
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

    pub fn key_points<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.key_points.iter().copied()
    }

    pub fn connections(&self) -> Vec<Connection<usize>> {
        vec_to_connections(&self.key_points)
    }

    pub fn move_mouse(&mut self, mouse_pos: Vec2<f32>, spell_grid: &SpellGrid) {
        if let Some(point) = spell_grid.point(mouse_pos - self.initial_mouse_pos) {
            if Some(&point) != self.key_points.last() {
                self.connect(point);
            }
        }
    }

    fn connect(&mut self, point: usize) {
        match self.key_points.last() {
            Some(&last) => {
                let new_connection = Connection {
                    from: last,
                    to: point,
                };
                self.add_connection(new_connection);
            }
            None => {
                self.key_points.push(point);
            }
        }
    }

    fn add_connection(&mut self, connection: Connection<usize>) {
        let connections = self.connections();
        if connections
            .iter()
            .any(|con| is_connection_part_of(&connection, &con))
        {
            return;
        }

        self.key_points.push(connection.to);
    }
}

fn is_connection_part_of(connection: &Connection<usize>, other: &Connection<usize>) -> bool {
    if other.from != 0 && other.to != 0 || connection.from != 0 && connection.to != 0 {
        return false;
    }

    let to = if connection.from == 0 {
        connection.to
    } else {
        connection.from
    };

    let delta = (other.from as i32 - other.to as i32).abs() - 1;
    if delta < 8 {
        return false;
    }

    let count = (delta / 8) as usize;
    for i in 0..count {
        let middle = 1 + i * 8;
        if to == middle {
            return true;
        }
    }

    false
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

fn normalize_connections<T: Ord>(mut connections: Vec<Connection<T>>) -> Vec<Connection<T>> {
    connections.sort();
    connections.dedup();
    connections
}

#[test]
fn connections() {
    let con1 = Connection { from: 0, to: 1 };
    let con2 = Connection { from: 0, to: 9 };
    assert!(is_connection_part_of(&con1, &con2));
    assert!(!is_connection_part_of(&con2, &con1));

    let con1 = Connection { from: 1, to: 0 };
    let con2 = Connection { from: 9, to: 0 };
    assert!(is_connection_part_of(&con1, &con2));
    assert!(!is_connection_part_of(&con2, &con1));

    let con1 = Connection { from: 1, to: 0 };
    let con2 = Connection { from: 0, to: 9 };
    assert!(is_connection_part_of(&con1, &con2));
    assert!(!is_connection_part_of(&con2, &con1));

    let con1 = Connection { from: 0, to: 1 };
    let con2 = Connection { from: 9, to: 0 };
    assert!(is_connection_part_of(&con1, &con2));
    assert!(!is_connection_part_of(&con2, &con1));

    let con1 = Connection { from: 1, to: 2 };
    let con2 = Connection { from: 9, to: 4 };
    assert!(!is_connection_part_of(&con1, &con2));
    assert!(!is_connection_part_of(&con2, &con1));
}
