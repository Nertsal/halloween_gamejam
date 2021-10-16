pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        assert!(max > 0.0);
        Self { current: max, max }
    }

    pub fn change(&mut self, delta: f32) -> bool {
        self.current += delta;
        self.current = self.current.clamp(0.0, self.max);
        self.current <= 0.0
    }

    pub fn fraction(&self) -> f32 {
        self.current / self.max
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}
