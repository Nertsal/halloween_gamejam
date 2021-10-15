use super::*;

impl GameState {
    pub fn update(&mut self, delta_time: f32) {
        self.control_player();
        self.control_skeletons();

        self.move_player(delta_time);
        self.move_skeletons(delta_time);
    }

    fn move_player(&mut self, delta_time: f32) {
        self.player.position += self.player.velocity * delta_time;
    }

    fn move_skeletons(&mut self, delta_time: f32) {
        for skeleton in &mut self.skeletons {
            skeleton.position += skeleton.velocity * delta_time;
        }
    }

    fn control_player(&mut self) {
        let mut move_x = 0.0;
        let mut move_y = 0.0;
        let window = self.geng.window();
        use geng::Key;
        if window.is_key_pressed(Key::W) {
            move_y += 1.0;
        }
        if window.is_key_pressed(Key::S) {
            move_y += -1.0;
        }
        if window.is_key_pressed(Key::A) {
            move_x += -1.0;
        }
        if window.is_key_pressed(Key::D) {
            move_x += 1.0;
        }

        let move_direction = vec2(move_x, move_y).clamp(1.0);
        self.player.velocity = self.player.speed * move_direction;
    }

    fn control_skeletons(&mut self) {}
}
