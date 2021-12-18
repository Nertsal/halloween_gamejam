pub struct Difficulty {
    time: f32,
    stage: u32,
}

impl Difficulty {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            stage: 0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
    }

    pub fn next_stage(&mut self) {
        self.stage += 1;
    }

    pub fn spawn_count(&self) -> usize {
        ((self.stage as f32).sqrt() as usize * 2).max(1)
    }
}
