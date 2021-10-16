use super::*;

pub struct Circle {
    pub position: Vec2<f32>,
    pub radius: f32,
}

impl Circle {
    pub fn new(position: Vec2<f32>, radius: f32) -> Self {
        Self { position, radius }
    }

    pub fn collision(&self, other: &Self) -> Option<Collision> {
        let delta = other.position - self.position;
        let distance = delta.len();
        if distance.approx_eq(&0.0) {
            return None;
        }

        let penetration = self.radius + other.radius - distance;
        if penetration > 0.0 {
            Some(Collision {
                normal: delta / distance,
                penetration,
            })
        } else {
            None
        }
    }

    pub fn bounds(&mut self, border: &AABB<f32>) {
        self.position.x = self
            .position
            .x
            .clamp(border.x_min + self.radius, border.x_max - self.radius);
        self.position.y = self
            .position
            .y
            .clamp(border.y_min + self.radius, border.y_max - self.radius);
    }
}
